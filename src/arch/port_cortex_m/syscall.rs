use crate::scheduler::with_scheduler;
use crate::signal::SignalType;
use core::arch::global_asm;

pub(crate) fn deal_syscall_exit() {}

pub(crate) fn deal_syscall_yield() {}

pub(crate) fn deal_syscall_sleep(time: usize) {
    with_scheduler(|s| s.delay_task(time));
}

pub(crate) fn deal_syscall_wait_signal(signal: SignalType) {
    with_scheduler(|s| s.block_task_with_signal(signal));
}

pub(crate) fn deal_syscall_send_signal(signal: SignalType) {
    with_scheduler(|s| s.send_signal(signal));
}

global_asm!(
    "    
    .syntax unified
    .section .text.asm
    .global call_task_exit
    .global call_task_yield
    .global call_task_sleep

    .global call_task_wait_signal
    .global call_task_send_signal

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

    .thumb_func
call_task_wait_signal:
    PUSH {{ LR }}
    svc 0x3
    POP {{ PC }}

    .thumb_func
call_task_send_signal:
    PUSH {{ LR }}
    svc 0x4
    POP {{ PC }}
    "
);
