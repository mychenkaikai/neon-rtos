#![no_std]
#![no_main]

mod common;
use crate::common::*;
mod interrupts;
mod lock;
mod port;
mod task;
use crate::port::*;
use crate::task::*;
extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc_cortex_m::CortexMHeap;
use core::panic::PanicInfo;
use cortex_m::peripheral::*;

#[panic_handler]
fn panic_halt(p: &PanicInfo) -> ! {
    hprintln!("{}", p);
    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    hprintln!("{:#?}", ef).ok();

    loop {}
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
pub fn init_heap() {
    unsafe {
        // 初始化堆，大小为 4096 字节（可以根据需要调整）
        ALLOCATOR.init(heap_start() as usize, 4096);
    }
}

use core::arch::asm;
use core::ptr;

#[exception]
unsafe fn DefaultHandler(val: i16) -> ! {
    let icsr = ptr::read_volatile(SCB::ptr() as *mut u32);
    hprintln!("DefaultHandler {} {}", val, (icsr >> 10) & 0x1FF);

    loop {}
}
#[exception]
fn SVCall() {
    hprintln!("SVCall ");
    vPortSVCHandler();
}

#[exception]
fn PendSV() {
    hprintln!("PendSV ");

    unsafe {
        vPortPenSVHandler();
    }
    loop {}
}

fn test1(arg: usize) {
    loop {
        hprintln!("123");
        taks_yeild();
    }
}
fn test2(arg: usize) {
    loop {
        hprintln!("456");
        taks_yeild();
    }
}

#[link_section = ".data"]
static bb: i32 = 1;
#[entry]
fn main() -> ! {
    let cc = 123;
    init_heap();
    // xTaskCreateStatic();
    // hprintln!("{}", create_task());

    create_task(test1, "123".to_string(), 500, 0);
    create_task(test2, "456".to_string(), 500, 0);

    hprintln!("{}", "1233");
    scheduler();

    unsafe {
        vPortStartFirstTask();
    }
    loop {}
}
