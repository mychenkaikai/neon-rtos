use crate::kernel::scheduler::with_scheduler;

pub fn create_task(
    name: &'static str,
    stack_size: usize,
    entry: fn(usize),
) -> Result<(), &'static str> {
    with_scheduler(|s| s.create_task(name, stack_size, entry))
}

pub fn start(ticks_per_second: usize) {
    with_scheduler(|s| s.start(ticks_per_second))
}
