extern "C" {
    pub(crate) fn call_task_exit();
    pub(crate) fn call_task_yield();
    pub(crate) fn call_task_sleep(time: usize);
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


