//! Types related to task management

use super::TaskContext;
use crate::{config::MAX_SYSCALL_NUM, timer::get_time_ms};

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Fisrt scheduled
    pub time_begin: Option<usize>,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

impl TaskControlBlock {
    /// Record syscall.
    pub fn record_syscall(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }

    /// Returns the record first time of this [`TaskControlBlock`].
    pub fn record_first_time(&mut self) {
        if self.time_begin == None {
            self.time_begin = Some(get_time_ms());
        }
    }

    /// Returns the real time of this [`TaskControlBlock`].
    pub fn real_time(&mut self) -> usize {
        if let Some(time_begin) = self.time_begin {
            get_time_ms().checked_sub(time_begin).unwrap()
        } else {
            0
        }
    }
}

impl Default for TaskControlBlock {
    fn default() -> Self {
        TaskControlBlock {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
            syscall_times: [0; MAX_SYSCALL_NUM],
            time_begin: None,
        }
    }
}
