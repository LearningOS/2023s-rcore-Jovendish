//! Process management syscalls
use core::mem::size_of;

use crate::mm::{MapPermission, VPNRange, VirtAddr, translated_byte_buffer};
use crate::task::{get_syscall_times, get_real_time, current_user_token};
use crate::timer::get_timeval;
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, insert_framed_area, remove_framed_area,
        suspend_current_and_run_next, translate, TaskStatus,
    },
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    get_timeval(ts);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let token = current_user_token();
    let mut buffers = translated_byte_buffer(token, ti as *const u8, size_of::<TaskInfo>());

    let taskinfo_ptr = buffers[0].as_mut_ptr() as *mut TaskInfo;
    let mut taskinfo = unsafe { &mut *taskinfo_ptr };

    taskinfo.status = TaskStatus::Running;
    taskinfo.syscall_times = get_syscall_times();
    taskinfo.time = get_real_time();

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    let start_va = VirtAddr::from(start);
    let end_va: VirtAddr = (start + len).into();

    // validity check
    // va must align to page, prot other bit must be zero and it is meaningless if low 3 bit is all zero.
    if start_va.page_offset() != 0 || prot & !0x7 != 0 || prot & 0x7 == 0 {
        return -1;
    }

    for vpn in VPNRange::new(start_va.floor(), end_va.ceil()) {
        let pte = translate(vpn);
        if pte.is_some() && pte.unwrap().is_valid() {
            return -1;
        }
    }

    if len == 0 {
        return 0;
    }

    let mut map_perm = MapPermission::U;

    if prot & 0b001 != 0 {
        map_perm |= MapPermission::R;
    }

    if prot & 0b010 != 0 {
        map_perm |= MapPermission::W;
    }

    if prot & 0b100 != 0 {
        map_perm |= MapPermission::X;
    }

    insert_framed_area(start_va, end_va, map_perm);

    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    let start_va = VirtAddr::from(start);
    let end_va: VirtAddr = (start + len).into();

    // validity check
    if start_va.page_offset() != 0 {
        return -1;
    }

    for vpn in VPNRange::new(start_va.floor(), end_va.ceil()) {
        let pte = translate(vpn);

        if pte.is_none() || !pte.unwrap().is_valid() {
            return -1;
        }
    }

    remove_framed_area(start_va, end_va);

    0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
