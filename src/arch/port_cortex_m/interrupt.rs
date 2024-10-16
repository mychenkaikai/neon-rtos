use crate::arch::port_cortex_m::syscall::*;
use crate::kernel::scheduler;
use crate::kernel_println;
use cortex_m_rt::exception;

#[exception]
unsafe fn SysTick() {
    scheduler::with_scheduler(|s| s.tick());
}

use cortex_m_rt::ExceptionFrame;

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    kernel_println!("{:#?}", ef);
    loop {}
}
#[exception]
unsafe fn DefaultHandler(_val: i16) -> ! {
    loop {}
}

#[no_mangle]
fn syscall_handler(args1: usize, args2: usize, args3: usize, svc_num: usize) {
    match svc_num {
        0 => syscall_exit(),
        1 => syscall_yield(),
        2 => syscall_sleep(args1 as usize),

        _ => panic!("syscall_handler: invalid svc_num: {}", svc_num),
    }
}
