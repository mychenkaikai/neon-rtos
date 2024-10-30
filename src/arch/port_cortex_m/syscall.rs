use crate::arch::common::ArchPortTrait;
use crate::arch::port::ArchPort;
use crate::scheduler::with_scheduler;
use crate::scheduler::TaskState;
use crate::signal::BlockReason;
use crate::signal::SignalType;
use crate::utils::double_list::Linkable;
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

pub(crate) fn deal_syscall_mutex_lock(mutex_id: usize) {
    let need_switch = with_scheduler(|s| {
        if !s.mutex_manager.lock(mutex_id, s.current_task.unwrap()) {
            // 获取锁失败，只改变当前任务状态
            if let Some(mut task) = s.current_task {
                task.state = TaskState::Blocked(BlockReason::Mutex(mutex_id));
            }
            true
        } else {
            false
        }
    });

    if need_switch {
        ArchPort::call_task_yield();
    }
}

pub(crate) fn deal_syscall_mutex_unlock(mutex_id: usize) {
    with_scheduler(|s| {
        if let Some(mut task) = s.mutex_manager.unlock(mutex_id) {
            // 只改变任务状态为就绪
            task.state = TaskState::Ready;
        }
    });
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
    .global call_task_mutex_lock
    .global call_task_mutex_unlock

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

    .thumb_func
call_task_mutex_lock:
    PUSH {{ LR }}
    svc 0x5
    POP {{ PC }}

    .thumb_func
call_task_mutex_unlock:
    PUSH {{ LR }}
    svc 0x6
    POP {{ PC }}
    "
);
