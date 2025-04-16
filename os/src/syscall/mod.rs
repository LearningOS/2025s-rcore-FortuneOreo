//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;

static mut WRITE_CALL_CNT: [usize; MAX_APP_NUM] = [0;MAX_APP_NUM];
static mut EXIT_CALL_CNT: [usize; MAX_APP_NUM] = [0;MAX_APP_NUM];
static mut YIED_CALL_CNT: [usize; MAX_APP_NUM] = [0;MAX_APP_NUM];
static mut GETTIME_CALL_CNT: [usize; MAX_APP_NUM] = [0;MAX_APP_NUM];
static mut TRACE_CALL_CNT: [usize; MAX_APP_NUM] = [0;MAX_APP_NUM];

mod fs;
mod process;

use fs::*;
use process::*;

use crate::{config::MAX_APP_NUM, task};

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    let current_task = task::get_id_of_current_task();

    match syscall_id {
        SYSCALL_WRITE => {
            unsafe { WRITE_CALL_CNT[current_task] += 1; }
            sys_write(args[0], args[1] as *const u8, args[2])
        },
        SYSCALL_EXIT => {
            unsafe { EXIT_CALL_CNT[current_task] += 1; }
            sys_exit(args[0] as i32)
        },
        SYSCALL_YIELD => {
            unsafe { YIED_CALL_CNT[current_task] += 1; }
            sys_yield()
        },
        SYSCALL_GET_TIME => {
            unsafe { GETTIME_CALL_CNT[current_task] += 1; }
            sys_get_time(args[0] as *mut TimeVal, args[1])
        },
        SYSCALL_TRACE => {
            unsafe { TRACE_CALL_CNT[current_task] += 1; }
            sys_trace(args[0], args[1], args[2])
        },
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

/// Get the syscall count number of the current task according to the syscall_id
pub fn get_task_syscall_count(syscall_id: usize) -> isize {
    let current_task = task::get_id_of_current_task();

    match syscall_id {
        SYSCALL_WRITE => unsafe { WRITE_CALL_CNT[current_task] as isize},
        SYSCALL_EXIT => unsafe { EXIT_CALL_CNT[current_task] as isize },
        SYSCALL_YIELD => unsafe { YIED_CALL_CNT[current_task] as isize },
        SYSCALL_GET_TIME => unsafe { GETTIME_CALL_CNT[current_task] as isize },
        SYSCALL_TRACE => unsafe { TRACE_CALL_CNT[current_task] as isize },
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
