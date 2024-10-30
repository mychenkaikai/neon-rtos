use crate::scheduler::with_scheduler;
use core::arch::global_asm;

pub(crate) fn deal_syscall_exit() {}

pub(crate) fn deal_syscall_yield() {}

pub(crate) fn deal_syscall_sleep(time: usize) {
    with_scheduler(|s| s.delay_task(time));
}
global_asm!(
    "
    .section .text.asm
    .global call_task_exit
    .global call_task_yield
    .global call_task_sleep

call_task_exit:
    li a7, 0
    ecall
    ret

call_task_yield:
    li a7, 1
    ecall
    ret

call_task_sleep:
    li a7, 2
    ecall
    ret
    "
);
