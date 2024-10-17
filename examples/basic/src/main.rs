#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_semihosting::hprintln;

use neon_rtos::syscall::*;
use neon_rtos::task;
use neon_rtos::utils::print;

const SYST_FREQ: u32 = 100;
const SYS_CLOCK: u32 = 12_000_000;
// 定义 SysTick 的重新加载值
const SYST_RELOAD: u32 = SYS_CLOCK / SYST_FREQ;

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

        task_sleep(500);
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

        task_sleep(1000);
    }
}

#[no_mangle]
fn app_main() -> ! {
    print::register_print_function(|msg| hprintln!("{}", msg));

    task::create_task("task1", 1024 * 2, test1).unwrap();
    task::create_task("task2", 1024 * 2, test2).unwrap();

    let f = || {
        let p = Peripherals::take().unwrap();
        let mut syst = p.SYST;

        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(SYST_RELOAD); // period = 10ms
        syst.enable_counter();
        syst.enable_interrupt();
    };

    task::start(f, SYST_FREQ as usize);
}
