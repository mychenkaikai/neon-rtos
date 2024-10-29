#![no_std]
#![no_main]

use core::panic::PanicInfo;

use semihosting::println;

use neon_rtos::syscall::*;
use neon_rtos::task;
use neon_rtos::utils::print;
use riscv::register::{mie, mip, mtimecmp, time, mstatus};
use riscv::register::mie::clear_mtimer;

const CLOCK_FREQ: u64 = 10_000_000; // 假设时钟频率为 10 MHz
const TICK_INTERVAL: u64 = CLOCK_FREQ / 100; // 10ms 间隔

#[panic_handler]
fn panic_halt(p: &PanicInfo) -> ! {
    println!("{}", p);
    loop {}
}

#[no_mangle]
fn machine_trap_handler() {
    // 检查是否是定时器中断
    if mip::read().mtimer() {
        // 清除定时器中断标志
        mip::clear_mtimer();

        // 设置下一次中断的时间
        unsafe {
            let timer = time::read64() + TICK_INTERVAL;
            mtimecmp::write64(timer);
        }

        // 在这里添加你的定时器中断处理代码
    }
    // 处理其他类型的中断或异常...
}

fn test1(_arg: usize) {
    loop {
        println!("task1");
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
        println!("task2");
        // let mut _a = 0;
        // for _ in 0..10000 {
        //     _a += 1;
        // }
        // ArchPort::task_yield();

        task_sleep(1000);
    }
}
const MTIME: *const u64 = 0x200BFF8 as *const u64;
const MTIMECMP: *mut u64 = 0x2004000 as *mut u64;
const TIMEBASE_FREQ: usize = 10000000;
#[no_mangle]
fn app_main() -> ! {
    print::register_print_function(|msg| println!("{}", msg));

    task::create_task("task1", 1024 * 2, test1).unwrap();
    task::create_task("task2", 1024 * 2, test2).unwrap();



    let f = || {
        *MTIMECMP = *MTIME + TIMEBASE_FREQ as u64;
        set_mtie();
    };

    task::start(f, 100 as usize);
}
