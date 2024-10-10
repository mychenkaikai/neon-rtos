pub mod mem;
use crate::{arch::common::*, kernel::scheduler};
use core::mem::size_of;
use core::{arch::asm, ops::Deref, ptr::addr_of};
use cortex_m;
pub struct ArchPort;

impl ArchPortTrait for ArchPort {
    fn idle_task() {
        cortex_m::asm::wfi();
    }
    fn enable_interrupts() {
        unsafe {
            cortex_m::interrupt::enable();
        }
    }
    fn disable_interrupts() {
        cortex_m::interrupt::disable();
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
    fn start_first_task() {
        unsafe {
            asm!(
                "ldr r0, =0xE000ED08",
                "ldr r0, [r0]",
                "ldr r0, [r0]",
                "msr msp, r0",
                "cpsie i",
                "cpsie f",
                "dsb",
                "isb",
                "svc 0",
            );
        }
    }
    fn syscall(number: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
        0
    }
    fn get_current_stack_pointer() -> *mut u8 {
        0 as *mut u8
    }
    fn set_stack_pointer(sp: *mut u8) {}
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
    #[inline]
    fn task_yield() {
        cortex_m::peripheral::SCB::set_pendsv();
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    fn init_task_stack(top_of_stack: &mut usize, func: fn(usize), p_args: usize) {
        unsafe {
            *top_of_stack &= !7;
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

use core::concat;
use cortex_m_semihosting::hprintln;

pub fn _println(args: core::fmt::Arguments) {
    hprintln!("{}", args);
}
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        crate::arch::port::_println(format_args!($($arg)*))
    };
}
pub use println;

fn task_exit_error() {
    println!("task_exit_error");
    loop {}
}
#[no_mangle]
fn stack_check() {
    unsafe {
        let mut r0: u32;
        asm!("mrs r0, psp", lateout("r0") r0);

        hprintln!("stack check start - PSP: 0x{:08x}", r0);
        for i in 8..16 {
            let value = *(r0 as *const u32).add(i);
            hprintln!("switch Stack[{}]: 0x{:08x}", i, value);
        }
        hprintln!("stack check end------------------");
    }
}

#[no_mangle]
fn stack_check_half() {
    unsafe {
        let mut r0: u32;

        asm!("mrs r0, psp", lateout("r0") r0);
        hprintln!("stack check start - PSP: 0x{:08x}", r0);
        for i in 0..8 {
            let value = *(r0 as *const u32).add(i);
            hprintln!("switch Stack[{}]: 0x{:08x}", i, value);
        }
        hprintln!("stack check end------------------");
    }
}

pub fn stack_check_context(psp: u32) {
    unsafe {
        for i in 8..16 {
            let value = *(psp as *const u32).add(i);
            hprintln!("switch Stack[{}]: 0x{:08x}", i, value);
        }
    }
}
#[no_mangle]
pub fn task_switch_context() {
    //stack check
    stack_check_half();
    // unsafe {
    //     let mut psp: u32;
    //     asm!("mrs {}, psp", out(reg) psp);
    //     psp &= !7; // 确保 8 字节对齐
    //     asm!("msr psp, {}", in(reg) psp);
    // }
    scheduler::with_scheduler(|s| s.task_switch_context());
    // unsafe {
    //     let mut psp: u32;
    //     asm!("mrs {}, psp", out(reg) psp);
    //     psp &= !7; // 确保 8 字节对齐
    //     asm!("msr psp, {}", in(reg) psp);
    // }
    // stack_check();
}

#[no_mangle]
fn port_svc_handler() {
    // unsafe {
    //     asm!(
    //         ".align 4",
    //         // "ldr r1, [r3]",
    //         // "ldr r0, [r1]",
    //         // in("r3") get_mut_current_task().unwrap().as_ptr(),
    //         in("r0") scheduler::with_scheduler(|s| s.current_task().unwrap().as_ptr()),
    //     );
    //     asm!(
    //         "ldmia r0!, {{r4-r11}}", // Pop the core registers
    //         "msr psp, r0",           // Pop the core registers

    //     );
    //     asm!(

    //         "isb",
    //         "mov r0, #0",
    //         "msr basepri, r0",
    //         "orr lr, #0xd",
    //     );
    //     asm!(        // Pop the core registers
    //         "bx lr",
    //         in("lr") 0xFFFFFFFD as usize
    //     );
    // }

    unsafe {
        asm!(
            ".align 4",
            // Load current task pointer
            "mov r0, {task_ptr}",
            "ldr r0, [r0]",

            // Restore registers from task stack



            task_ptr = in(reg) scheduler::with_scheduler(|s| s.current_task().unwrap().as_ptr()),

            options(nostack)
        );
        asm!(
            "ldmia r0!, {{r4-r11}}",
            "msr psp, r0",
            "isb",
            // Clear BASEPRI to enable all interrupts
            "mov r0, #0",
            "msr basepri, r0",
            // Set LR for return to Thread mode using PSP
            "mov lr, 0xFFFFFFFD",
            // 保存关键寄存器

            // Return to Thread mode, switching to PSP
            options(nostack)
        );
        //read psp
        // asm!("mrs r0, psp");
        // let psp: u32;
        // asm!("mrs {}, psp", out(reg) psp);
        // hprintln!("SVC Handler Start - PSP: 0x{:08x}", psp);
        // for i in 0..8 {
        //     let value = *(psp as *const u32).add(i);
        //     hprintln!("Stack[{}]: 0x{:08x}", i, value);
        // }
        asm!("bx lr");
    }
}

#[no_mangle]
fn port_pendsv_handler() {
    unsafe {
        asm!("mrs r0, psp");
        asm!("add  sp, #8");
        // asm!("mrs r0, psp");
        asm!(
            "mrs r0, psp",
            "isb",
            // "ldr r2, [r3]",
            // "bic r0, r0, #7",

            "stmdb r0!, {{r4-r11}}",

            // "ldr r2, [r3]",
            "str r0, [r3]",
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

        asm!(
        "ldr r0, [r3]",
        in("r3") scheduler::with_scheduler(|s| s.current_task().unwrap().as_ptr()),
        );

        asm!(
            "ldmia r0!, {{r4-r11}}",
            "msr psp, r0",
            "isb",
            "push {{r0, r1, r2, r3, r12, lr}}",
            // 调用堆栈检查函数
            "bl stack_check_half",
            // 恢复关键寄存器
            "pop {{r0, r1, r2, r3, r12, lr}}",
        );

        asm!(        // Pop the core registers
            "bx lr",
            in("lr") 0xFFFFFFFD as usize
        );
    }
}
