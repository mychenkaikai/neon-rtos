#![no_std]
#![no_main]

// extern crate neon;
use neon::common::common::*;
// use neon::port::port::*;
use neon::arch_port::common::ArchPortTrait;
use neon::arch_port::port::ArchPort;
use neon::kernel::scheduler::{self, *};
use neon::println;

use alloc_cortex_m::CortexMHeap;
use core::panic::PanicInfo;
#[panic_handler]
fn panic_halt(p: &PanicInfo) -> ! {
    println!("{}", p);
    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    println!("{:#?}", ef);
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
    println!("DefaultHandler ");
    loop {}
}

#[exception]
unsafe fn SysTick() {
    scheduler::with_scheduler(|s| s.tick());
}

fn test1(_arg: usize) {
    loop {
        println!("task1");
        let mut _a = 0;
        for _ in 0..10000000 {
            _a += 1;
        }
        ArchPort::task_yield();
        // task_delay(500);
    }
}
fn test2(_arg: usize) {
    loop {
        hprintln!("task2");
        let mut _a = 0;
        for _ in 0..10000000 {
            _a += 1;
        }
        ArchPort::task_yield();
        // task_delay(1000);
    }
}
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
// 定义 SysTick 的频率（假设为 1 kHz）

const SYST_FREQ: u32 = 100;
const SYS_CLOCK: u32 = 12_000_000;
// 定义 SysTick 的重新加载值
const SYST_RELOAD: u32 = SYS_CLOCK / SYST_FREQ;

#[entry]
fn main() -> ! {
    init_heap();

    let mut scheduler = Scheduler::new();
    scheduler.create_task("task1", 1000, test1).unwrap();
    scheduler.create_task("task2", 1000, test2).unwrap();
    scheduler.start();

    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYST_RELOAD); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    ArchPort::start_first_task();

    loop {}
}
