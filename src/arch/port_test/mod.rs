pub mod mem;
use crate::arch_port::common::*;

pub struct ArchPort;

impl ArchPortTrait for ArchPort {
    fn idle_task() { /* 实现 */
    }
    fn enable_interrupts() { /* 实现 */
    }
    fn disable_interrupts() { /* 实现 */
    }
    fn is_interrupts_enabled() -> bool {
        true
    }
    fn enter_critical_section() { /* 实现 */
    }
    fn exit_critical_section() { /* 实现 */
    }
    fn get_system_tick_count() -> u64 {
        0
    }
    fn delay_ms(ms: u32) { /* 实现 */
    }
    fn memory_barrier() { /* 实现 */
    }
    fn trigger_context_switch() { /* 实现 */
    }
    fn start_first_task() { /* 实现 */
    }
    fn syscall(number: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
        0
    }
    fn get_current_stack_pointer() -> *mut u8 {
        0 as *mut u8
    }
    fn set_stack_pointer(sp: *mut u8) { /* 实现 */
    }
    fn get_current_privilege_level() -> u8 {
        0
    }
    fn switch_to_user_mode() { /* 实现 */
    }
    fn invalidate_instruction_cache() { /* 实现 */
    }
    fn flush_data_cache() { /* 实现 */
    }
    fn enter_low_power_mode() { /* 实现 */
    }
    fn exit_low_power_mode() { /* 实现 */
    }
    fn set_exception_handler(exception_type: ExceptionType, handler: fn()) { /* 实现 */
    }
    fn get_last_exception_info() -> ExceptionInfo {
        ExceptionInfo::new()
    }
    fn get_cpu_id() -> u32 {
        0
    }
    fn get_core_count() -> u32 {
        0
    }
    fn task_yield() {
        crate::kernel::scheduler::with_scheduler(|scheduler| scheduler.task_switch_context());
    }

    fn init_task_stack(top_of_stack: &mut usize, func: fn(usize), p_args: usize) {}
}
