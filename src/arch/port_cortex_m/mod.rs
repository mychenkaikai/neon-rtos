pub mod asm;
pub mod interrupt;
pub mod mem;
pub mod syscall;
use crate::kernel::scheduler::with_scheduler;
use crate::{arch::common::*, kernel::scheduler};

use core::mem::size_of;
use core::{arch::asm, ptr::addr_of};
use cortex_m::{self, register::psp};

use crate::kernel_println;

pub struct ArchPort;

impl ArchPortTrait for ArchPort {
    #[inline]
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
    fn enter_critical_section() {
        cortex_m::interrupt::disable();
    }
    fn exit_critical_section() {
        unsafe {
            cortex_m::interrupt::enable();
        }
    }
    #[inline]
    fn critical_section<F: FnOnce()>(func: F) {
        cortex_m::interrupt::free(|_| {
            func();
        });
    }

    fn delay_ms(ms: u32) {}
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
                // "svc 0",
            );
        }
        ArchPort::task_yield();
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

fn task_exit_error() {
    kernel_println!("task_exit_error");
    loop {}
}

#[no_mangle]
fn stack_check() {
    unsafe {
        let mut r0: u32;
        asm!("mrs r0, psp", lateout("r0") r0);

        kernel_println!("stack check start - PSP: 0x{:08x}", r0);
        for i in 8..16 {
            let value = *(r0 as *const u32).add(i);
            kernel_println!("switch Stack[{}]: 0x{:08x}", i, value);
        }
        kernel_println!("stack check end------------------");
    }
}

#[no_mangle]
fn stack_check_half() {
    unsafe {
        let mut r0: u32;

        asm!("mrs r0, psp", lateout("r0") r0);
        kernel_println!("stack check start - PSP: 0x{:08x}", r0);
        for i in 0..8 {
            let value = *(r0 as *const u32).add(i);
            kernel_println!("switch Stack[{}]: 0x{:08x}", i, value);
        }
        kernel_println!("stack check end------------------");
    }
}

pub fn stack_check_context(psp: u32) {
    unsafe {
        for i in 8..16 {
            let value = *(psp as *const u32).add(i);
            kernel_println!("switch Stack[{}]: 0x{:08x}", i, value);
        }
    }
}

#[no_mangle]
pub fn task_switch_context() {
    scheduler::with_scheduler(|s| s.task_switch_context());
}

#[no_mangle]
#[inline]
pub fn set_current_task_stack_top(psp: *mut u32) {
    with_scheduler(|s| {
        s.current_task().map(|mut tcb| {
            tcb.stack_top = psp as usize;
        });
    });
}
#[no_mangle]
#[inline]
pub fn get_current_task_stack_top() -> *mut u32 {
    with_scheduler(|s| s.current_task().map(|tcb| tcb.stack_top as *mut u32)).unwrap()
}

pub fn set_psp(psp: usize) {
    unsafe {
        psp::write(psp as u32);
    }
}

pub fn get_psp() -> usize {
    psp::read() as usize
}
