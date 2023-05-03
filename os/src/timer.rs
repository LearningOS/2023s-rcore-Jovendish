//! RISC-V timer-related functionality

use core::mem::size_of;

use crate::config::CLOCK_FREQ;
use crate::mm::translated_byte_buffer;
use crate::sbi::set_timer;
use crate::syscall::TimeVal;
use crate::task::current_user_token;
use riscv::register::time;
/// The number of ticks per second
const TICKS_PER_SEC: usize = 100;
#[allow(dead_code)]
/// The number of milliseconds per second
const MSEC_PER_SEC: usize = 1000;
/// The number of microseconds per second
#[allow(dead_code)]
const MICRO_PER_SEC: usize = 1_000_000;

/// Get the current time in ticks
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

pub fn get_timeval(ts: *mut TimeVal) {
    let token = current_user_token();
    let mut buffers = translated_byte_buffer(token, ts as *const u8, size_of::<TimeVal>());

    let timeval_ptr = buffers[0].as_mut_ptr() as *mut TimeVal;
    let mut timeval = unsafe { &mut *timeval_ptr };

    timeval.sec  = get_time_ms() / MSEC_PER_SEC;
    timeval.usec = get_time_us() % MICRO_PER_SEC;
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
