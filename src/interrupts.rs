extern crate cortex_m;
use cortex_m::interrupt;
// use core::arch::asm;
#[allow(dead_code)]
/// 关闭中断
pub fn disable_interrupts() {
    interrupt::disable();
}
#[allow(dead_code)]
/// 恢复中断
pub fn enable_interrupts() {
    unsafe {
        interrupt::enable();
    }
}
