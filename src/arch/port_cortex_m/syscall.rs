use crate::kernel::scheduler::with_scheduler;
use core::arch::global_asm;

pub fn deal_syscall_exit() {}

pub fn deal_syscall_yield() {}

pub fn deal_syscall_sleep(time: usize) {
    with_scheduler(|s| s.delay_task(time));
}

global_asm!(
    "    
    .syntax unified
    .section .text.asm
    .global call_task_exit
    .global call_task_yield
    .global call_task_sleep
    

    .thumb_func
call_task_exit:
    PUSH {{ LR }}
    svc 0x0
    POP {{ PC }}

    .thumb_func
call_task_yield:
    PUSH {{ LR }}
    svc 0x1
    POP {{ PC }}

    .thumb_func
call_task_sleep:
    PUSH {{ LR }}
    svc 0x2
    POP {{ PC }}

    "
);
