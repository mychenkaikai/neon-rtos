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

fn test1(_arg: usize) {
    loop {
        taks_yeild!();
        hprintln!("123").unwrap();

        let mut _a = 0;
        for _ in 0..10000000 {
            _a += 1;
        }
    }
}
fn test2(_arg: usize) {
    loop {
        taks_yeild!();
        hprintln!("456").unwrap();
        let mut _a = 0;
        for _ in 0..10000000 {
            _a += 1;
        }
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
