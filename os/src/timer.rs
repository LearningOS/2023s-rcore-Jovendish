//! RISC-V timer-related functionality

use riscv::register::time;

use crate::mm::translated_refmut;
use crate::sbi::set_timer;
use crate::syscall::TimeVal;
use crate::task::current_user_token;

const TICKS_PER_SEC   : usize = 100;
const MILLI_PER_SEC   : usize = 1000;
const MICRO_PER_SEC   : usize = 1000000;
const CLOCK_FREQ      : usize = 12500000;
const CLOCKS_PER_MSEC : usize = CLOCK_FREQ / MILLI_PER_SEC;

/// Get the current time in ticks
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    get_time() / CLOCKS_PER_MSEC
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    get_time_ms() * MILLI_PER_SEC
}

/// get current time in timeval.
pub fn get_timeval(ts: *mut TimeVal) {
    let token = current_user_token();
    let timeval = translated_refmut(token, ts);

    timeval.sec  = get_time_ms() / MILLI_PER_SEC;
    timeval.usec = get_time_us() % MICRO_PER_SEC;
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
