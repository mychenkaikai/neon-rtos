use crate::task::CURRENT_TASK;
use core::arch::asm;
use core::ptr::addr_of;
/// 启动第一个任务
pub unsafe fn v_port_start_first_task() {
    asm!(
        "ldr r0, =0xE000ED08
         ldr r0, [r0]
         ldr r0, [r0]",
        "msr msp, r0",
        "cpsie i",
        "cpsie f",
        "dsb",
        "isb",
        "svc 0",
    );
}
// #[exception]
#[no_mangle]
fn port_svc_handler() {
    unsafe {
        asm!(
            ".align 4",
            "ldr r1, [r3]",
            "ldr r0, [r1]",
            in("r3") addr_of!(CURRENT_TASK) as usize + 4,
        );
        asm!(
            "ldmia r0!, {{r4-r11}}", // Pop the core registers
            "msr psp, r0",           // Pop the core registers
            "isb",
            "mov r0, #0",
            "msr basepri, r0",
            "orr lr, #0xd",
        );
        asm!(        // Pop the core registers
            "bx lr",
            in("lr") 0xFFFFFFFD as usize
        );
    }
}

#[no_mangle]
fn port_pendsv_handler() {
    unsafe {
        {
            asm!("add  sp, #16");

            asm!(
                "mrs r0, psp",
                "isb",
                "ldr r2, [r3]",
                "stmdb r0!, {{r4-r11}}",
                "str r0, [r2]",
                in("r3") addr_of!(CURRENT_TASK) as usize + 4,
            );
        }

        asm!(
            "stmdb sp!, {{r3, r14}}",
            /* configMAX_SYSCALL_INTERRUPT_PRIORITY*/
            "mov r0, #0",
            "msr basepri, r0",
            "dsb",
            "isb",
            "bl task_switch_context",
            "mov r0, #0",
            "msr basepri, r0",
            "ldmia sp!, {{r3, r14}}",
        );

        {
            asm!("ldr r1, [r3]",
            "ldr r0, [r1]",
            in("r3") addr_of!(CURRENT_TASK) as usize + 4,
            );
        }
        asm!("ldmia r0!, {{r4-r11}}", "msr psp, r0", "isb",);

        asm!(        // Pop the core registers
            "bx lr",
            in("lr") 0xFFFFFFFD as usize
        );
    }
}
