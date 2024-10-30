// pub mod cpu;
// pub mod interrupt;


use core::ptr::NonNull;

pub trait ArchPortTrait {
    fn idle_task();
    fn enable_interrupts();
    fn disable_interrupts();
    fn is_interrupts_enabled() -> bool;
    fn enter_critical_section();
    fn exit_critical_section();
    fn critical_section<F: FnOnce()>(func: F);

    fn delay_ms(ms: u32);
    fn memory_barrier();

    fn start_first_task();

    fn call_task_yield() {}

    fn init_task_stack(top_of_stack: &mut usize, func: fn(usize), p_args: usize) {}
}

pub trait MemOperations {
    fn mem_alloc(size: usize) -> *mut u8;
    fn mem_free(ptr: *mut u8);
    fn type_malloc<T>(data: T) -> NonNull<T>;
    fn type_free<T>(ptr: NonNull<T>) -> T;
}

pub enum ExceptionType {
    // 定义异常类型
}

pub struct ExceptionInfo {
    // 定义异常信息结构
}

impl ExceptionInfo {
    pub fn new() -> Self {
        ExceptionInfo {
            // 初始化异常信息结构
        }
    }
}