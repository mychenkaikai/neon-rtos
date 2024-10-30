use crate::arch::common::ArchPortTrait;
use crate::arch::port::ArchPort;
use crate::kernel::scheduler::*;
use crate::kernel::sync::signal::*;
use crate::kernel::task::tcb::*;
use crate::utils::double_list::Linkable;

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
