#![no_std]
#![no_main]

mod common;
use crate::common::*;
mod interrupts;
mod port;
mod task;
use crate::port::*;
use crate::task::*;
extern crate alloc;
use alloc::string::ToString;
use alloc_cortex_m::CortexMHeap;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_halt(p: &PanicInfo) -> ! {
    hprintln!("{}", p).unwrap();
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

#[exception]
unsafe fn DefaultHandler(_val: i16) -> ! {
    hprintln!("DefaultHandler ").unwrap();

    loop {}
}
#[exception]
fn SVCall() {
    // hprintln!("SVCall ");
    v_port_svc_handler();
}

#[exception]
fn PendSV() {
    // hprintln!("PendSV ");

    v_port_pensv_handler();

    loop {}
}

fn test1(_arg: usize) {
    loop {
        hprintln!("123").unwrap();
        taks_yeild();
    }
}
fn test2(_arg: usize) {
    loop {
        hprintln!("456").unwrap();
        taks_yeild();
    }
}

// #[link_section = ".data"]
// static bb: i32 = 1;
#[entry]
fn main() -> ! {
    init_heap();

    create_task(test1, "123".to_string(), 500, 0).unwrap();
    create_task(test2, "456".to_string(), 500, 0).unwrap();

    scheduler();

    unsafe {
        v_port_start_first_task();
    }
    loop {}
}
