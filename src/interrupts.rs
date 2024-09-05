extern crate cortex_m;
use cortex_m::interrupt;
// use core::arch::asm;

/// 关闭中断
pub fn disable_interrupts() {
    unsafe {
        interrupt::disable();
    }
}

/// 恢复中断
pub fn enable_interrupts() {
    unsafe {
        interrupt::enable();
    }
}
