extern "C" {
    pub fn call_task_exit();
    pub fn call_task_yield();
    pub fn call_task_sleep(time: usize);
    pub fn call_task_create(name: &'static str, stack_size: usize, entry: fn(usize));
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


