use crate::kernel::scheduler::with_scheduler;

extern "C" {
    pub fn task_exit();
    pub fn task_yield();
    pub fn task_sleep(time: usize);
}

pub fn syscall_exit() {}

pub fn syscall_yield() {}

pub fn syscall_sleep(time: usize) {
    with_scheduler(|s| s.delay_task(time));
}
