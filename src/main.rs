#![no_std]
#![no_main]

mod common;
use crate::common::*;
mod interrupts;
mod list;
mod mem;
mod port;
mod task;
use crate::port::*;
use crate::task::*;
extern crate alloc;

use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::panic::PanicInfo;
use core::ptr::addr_of;
use core::ptr::addr_of_mut;
use core::ptr::NonNull;
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
unsafe fn SysTick() {
    cortex_m::interrupt::free(|_cs| {
        systick_task_inc();
    });
}

fn test1(_arg: usize) {
    loop {
        hprintln!("task1").unwrap();
        let mut _a = 0;
        for _ in 0..10000000 {
            _a += 1;
        }
        // task_yield!();
        task_delay(1000);
    }
}
fn test2(_arg: usize) {
    loop {
        hprintln!("task2").unwrap();
        let mut _a = 0;
        for _ in 0..10000000 {
            _a += 1;
        }
        // task_yield!();
        task_delay(1000);
    }
}
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
// 定义 SysTick 的频率（假设为 1 kHz）

const SYST_FREQ: u32 = 100;
const SYS_CLOCK: u32 = 12_000_000;
// 定义 SysTick 的重新加载值
const SYST_RELOAD: u32 = SYS_CLOCK / SYST_FREQ;

use alloc::collections::LinkedList;
use core::cell::UnsafeCell;
static mut TT: UnsafeCell<LinkedList<TCB>> = UnsafeCell::new(LinkedList::new());

#[entry]
fn main() -> ! {
    init_heap();

    unsafe {
        TT.get_mut().push_back(TCB {
            top_of_stack: 0,
            stack_addr: 0,
            name: "none",
            stack_size: 0,
            prev: None,
            next: None,
            item_value: None,
            self_handle: NonNull::dangling(),
            list_handle: None,
        });
        TT.get_mut().back_mut().unwrap().self_handle = NonNull::new_unchecked
        // (((TT.get_mut().back().unwrap())as *const TCB as *mut TCB));
        ((addr_of_mut!((*TT.get_mut().back_mut().unwrap()))));
    }

    create_task(test1, "task1", 500, 0).unwrap();
    create_task(test2, "task2", 500, 0).unwrap();

    scheduler();
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYST_RELOAD); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    unsafe {
        v_port_start_first_task();
    }
    loop {}
}
