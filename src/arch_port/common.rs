pub trait ArchPortTrait {
    fn idle_task();
    fn enable_interrupts();
    fn disable_interrupts();
    fn is_interrupts_enabled() -> bool;
    fn enter_critical_section();
    fn exit_critical_section();
    fn get_system_tick_count() -> u64;
    fn delay_ms(ms: u32);
    fn memory_barrier();
    fn trigger_context_switch();
    fn start_first_task();
    fn syscall(number: usize, arg1: usize, arg2: usize, arg3: usize) -> usize;
    fn get_current_stack_pointer() -> *mut u8;
    fn set_stack_pointer(sp: *mut u8);
    fn get_current_privilege_level() -> u8;
    fn switch_to_user_mode();
    fn invalidate_instruction_cache();
    fn flush_data_cache();
    fn enter_low_power_mode();
    fn exit_low_power_mode();
    fn set_exception_handler(exception_type: ExceptionType, handler: fn());
    fn get_last_exception_info() -> ExceptionInfo;
    fn get_cpu_id() -> u32;
    fn get_core_count() -> u32;
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