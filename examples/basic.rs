#![no_std]
#![no_main]

use core::panic::PanicInfo;
use neon_rtos::arch::common::ArchPortTrait;
use neon_rtos::arch::port::syscall::*;
use neon_rtos::arch::port::*;
use neon_rtos::task;
use neon_rtos::utils::print;

use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic_halt(p: &PanicInfo) -> ! {
    hprintln!("{}", p);
    loop {}
}

fn test1(_arg: usize) {
    loop {
        hprintln!("task1");
        // let mut _a = 0;
        // for _ in 0..10000 {
        //     _a += 1;
        // }
        // ArchPort::task_yield();

        unsafe {
            task_sleep(500);
        }
    }
}
fn test2(_arg: usize) {
    loop {
        hprintln!("task2");
        // let mut _a = 0;
        // for _ in 0..10000 {
        //     _a += 1;
        // }
        // ArchPort::task_yield();

        unsafe {
            task_sleep(1000);
        }
    }
}

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
// 定义 SysTick 的频率（假设为 1 kHz）

const SYST_FREQ: u32 = 100;
const SYS_CLOCK: u32 = 12_000_000;
// 定义 SysTick 的重新加载值
const SYST_RELOAD: u32 = SYS_CLOCK / SYST_FREQ;

#[no_mangle]
fn app_main() -> ! {
    print::register_print_function(|msg| hprintln!("{}", msg));

    task::create_task("task1", 1024 * 2, test1).unwrap();
    task::create_task("task2", 1024 * 2, test2).unwrap();
    task::start(SYST_FREQ as usize);

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
