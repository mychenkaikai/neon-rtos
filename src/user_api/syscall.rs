use crate::kernel::sync::signal::SignalType;

extern "C" {
    pub(crate) fn call_task_exit();
    pub(crate) fn call_task_yield();
    pub(crate) fn call_task_sleep(time: usize);
    pub(crate) fn call_task_wait_signal(signal: usize);
    pub(crate) fn call_task_send_signal(signal: usize);
    pub(crate) fn call_task_mutex_lock(mutex_id: usize);
    pub(crate) fn call_task_mutex_unlock(mutex_id: usize);
}

pub fn task_exit() {
    unsafe {
        call_task_exit();
    }
}

pub fn task_yield() {
    unsafe {
        call_task_yield();
    }
}

pub fn task_sleep(time: usize) {
    unsafe {
        call_task_sleep(time);
    }
}

pub fn task_wait_signal(signal: SignalType) {
    unsafe {
        call_task_wait_signal(signal.into());
    }
}

pub fn task_send_signal(signal: SignalType) {
    unsafe {
        call_task_send_signal(signal.into());
    }
}

pub fn task_mutex_lock(mutex_id: usize) {
    unsafe {
        call_task_mutex_lock(mutex_id);
    }
}

pub fn task_mutex_unlock(mutex_id: usize) {
    unsafe {
        call_task_mutex_unlock(mutex_id);
    }
}
