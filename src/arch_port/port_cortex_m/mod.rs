pub mod mem;
use crate::{arch_port::common::*, kernel::scheduler};
use core::{arch::asm, ops::Deref, ptr::addr_of};
use cortex_m;
use core::mem::size_of;
pub struct ArchPort;

impl ArchPortTrait for ArchPort {
    fn idle_task() {
        cortex_m::asm::wfi();
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
    fn set_stack_pointer(sp: *mut u8) {
        unsafe {
            asm!("msr psp, r0", "isb");
        }
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
        cortex_m::peripheral::SCB::set_pendsv();
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    fn init_task_stack(top_of_stack: &mut usize, func: fn(usize), p_args: usize) {
        unsafe {
            *top_of_stack -= 1 * size_of::<usize>();
            *(*top_of_stack as *mut usize) = 0x0100_0000;
            *top_of_stack -= 1 * size_of::<usize>();
            *(*top_of_stack as *mut usize) = 0xffff_fffe & (func as usize);
            *top_of_stack -= 1 * size_of::<usize>();
            *(*top_of_stack as *mut usize) = task_exit_error as usize;
            *top_of_stack -= 5 * size_of::<usize>();
            *(*top_of_stack as *mut usize) = p_args;
            *top_of_stack -= 8 * size_of::<usize>();
        }
    }
}

use cortex_m_semihosting::hprintln;
use core::concat;
#[macro_export]
macro_rules! println {
    () => {
        hprintln!()
    };
    ($($arg:tt)*) => {
        hprintln!($($arg)*)
    };
}

fn task_exit_error() {
    println!("task_exit_error");
    loop {}
}

#[no_mangle]
pub fn task_switch_context() {
    scheduler::with_scheduler(|s| s.task_switch_context());
}

#[no_mangle]
fn port_svc_handler() {
    unsafe {
        asm!(
            ".align 4",
            "ldr r1, [r3]",
            "ldr r0, [r1]",
            // in("r3") get_mut_current_task().unwrap().as_ptr(),
            in("r3") scheduler::with_scheduler(|s| s.current_task().unwrap().as_ptr()),
        );
        asm!(
            "ldmia r0!, {{r4-r11}}", // Pop the core registers
            "msr psp, r0",           // Pop the core registers
            "isb",
            "mov r0, #0",
            "msr basepri, r0",
            "orr lr, #0xd",
        );
        asm!(        // Pop the core registers
            "bx lr",
            in("lr") 0xFFFFFFFD as usize
        );
    }
}

#[no_mangle]
fn port_pendsv_handler() {
    unsafe {
        asm!("add  sp, #16");

        asm!(
            "mrs r0, psp",
            "isb",
            "ldr r2, [r3]",
            "stmdb r0!, {{r4-r11}}",
            "str r0, [r2]",
            in("r3") scheduler::with_scheduler(|s| s.current_task().unwrap().as_ptr()),
        );

        asm!(
            "stmdb sp!, {{r3, r14}}",
            /* configMAX_SYSCALL_INTERRUPT_PRIORITY*/
            "mov r0, #0",
            "msr basepri, r0",
            "dsb",
            "isb",
            "bl task_switch_context",
            "mov r0, #0",
            "msr basepri, r0",
            "ldmia sp!, {{r3, r14}}",
        );

        asm!("ldr r1, [r3]",
        "ldr r0, [r1]",
        in("r3") scheduler::with_scheduler(|s| s.current_task().unwrap().as_ptr()),
        );

        asm!("ldmia r0!, {{r4-r11}}", "msr psp, r0", "isb",);

        asm!(        // Pop the core registers
            "bx lr",
            in("lr") 0xFFFFFFFD as usize
        );
    }
}
