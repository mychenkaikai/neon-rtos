pub mod mem;
use crate::kernel::scheduler::with_scheduler;
use crate::{arch::common::*, kernel::scheduler};
use core::arch::global_asm;
use core::mem::size_of;
use core::{arch::asm, ops::Deref, ptr::addr_of};
use cortex_m;
use cortex_m_rt::exception;

// include!("cortex_m.s");
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

    fn delay_ms(ms: u32) { /* 实现 */
    }
    fn memory_barrier() { /* 实现 */
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
    scheduler::with_scheduler(|s| s.task_switch_context());
}

#[no_mangle]
#[inline]
pub fn set_current_task_psp(psp: *mut u32) {
    with_scheduler(|s| {
        s.current_task().map(|mut t| {
            t.stack_top = psp as usize;
        });
    });
}
#[no_mangle]
#[inline]
pub fn get_current_task_psp() -> *mut u32 {
    with_scheduler(|s| s.current_task().map(|t| t.stack_top as *mut u32)).unwrap()
}

global_asm!(
    "
    .syntax unified
    .cpu cortex-m4
    .thumb

    .global SVC_Handler
    .type SVC_Handler, %function

SVC_Handler:
    cpsid i                  @ 禁用中断

    @ 恢复新任务的上下文
    bl get_current_task_psp  @ 获取新任务的 PSP
    ldmia r0!, {{r4-r11}}
    msr psp, r0

    @ 清除 BASEPRI 以启用所有中断
    mov r0, #0
    msr basepri, r0

    cpsie i                  @ 启用中断
    mov lr, #0xFFFFFFFD      @ 设置 LR 以使用 PSP 返回到线程模式
    bx lr

    .size SVC_Handler, .-SVC_Handler
    "
);

global_asm!(
    "    
    .syntax unified
    .cpu cortex-m4
    .thumb

    .global PendSV_Handler
    .type PendSV_Handler, %function

PendSV_Handler:
    cpsid i                  @ 禁用中断

    @ 保存当前上下文
    mrs r0, psp
    stmdb r0!, {{r4-r11}}
    bl set_current_task_psp  @ 调用 Rust 函数保存 PSP

    @ 执行任务切换
    bl task_switch_context   @ 调用 Rust 函数进行任务切换

    @ 恢复新任务的上下文
    bl get_current_task_psp  @ 获取新任务的 PSP
    ldmia r0!, {{r4-r11}}
    msr psp, r0

    cpsie i                  @ 启用中断
    mov lr, #0xFFFFFFFD
    bx lr

    .size PendSV_Handler, .-PendSV_Handler"
);
