use crate::scheduler;
use crate::kernel_println;

use core::arch::global_asm;

use super::syscall::*;

#[no_mangle]
fn syscall_handler(args1: usize, args2: usize, args3: usize, svc_num: usize) {
    match svc_num {
        0 => deal_syscall_exit(),
        1 => deal_syscall_yield(),
        2 => deal_syscall_sleep(args1 as usize),

        _ => panic!("syscall_handler: invalid svc_num: {}", svc_num),
    }
}
