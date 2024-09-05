use core::arch::asm;
/// 启动第一个任务
pub unsafe fn v_port_start_first_task() {
    // 使用 NVIC 偏移寄存器定位栈
    let mut r0: u32;
    asm!(
        "ldr r0, =0xE000ED08
         ldr r0, [r0]
         ldr r0, [r0]",
        out("r0") r0
    );

    // 设置 msp 回到栈的起始位置
    asm!(
        "msr msp, r0",
        in("r0") r0
    );

    // 调用 SVC 来启动第一个任务，确保中断被启用
    asm!(
        "
        cpsie i
        cpsie f
        dsb
        isb
        svc 0"
    );
}

pub extern "C" fn v_port_svc_handler() {
    unsafe {

        let tmp = &(crate::task::CURRENT_TASK.unwrap()) as *const *const TCB;

        asm!(
            ".align 4",
            "ldr r1, [r3]",
            "ldr r0, [r1]",
            in("r3") tmp,
        );
        asm!(
            "ldmia r0!, {{r4-r11}}",          // Pop the core registers
        );

        asm!(
            "msr psp, r0",        // Pop the core registers
        );

        asm!(
            // Pop the core registers
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

// global_asm!(include_str!("port.s"));

// extern "C"{
//     pub fn vPortPenSVHandler();

use crate::TCB;

pub extern "C" fn v_port_pensv_handler() {
    unsafe {
        let tmp = &(crate::task::CURRENT_TASK.unwrap()) as *const *const TCB;

        asm!("mrs r0, psp",
            "isb",
            "ldr r2, [r3]",
            "stmdb r0!, {{r4-r11}}",
            "str r0, [r2]",
            in("r3") tmp,
        );
        asm!(
            "stmdb sp!, {{r3, r14}}",
            /* configMAX_SYSCALL_INTERRUPT_PRIORITY*/
        );
        asm!(
            "mov r0, #0",
            "msr basepri, r0",
            "dsb",
            "isb",
            "bl task_switch_context",
        );

        asm!("mov r0, #0", "msr basepri, r0", "ldmia sp!, {{r3, r14}}",);

        let tmp1 = &(crate::task::CURRENT_TASK.unwrap()) as *const *const TCB;
        asm!("ldr r1, [r3]", "ldr r0, [r1]",in("r3") tmp1);
        asm!("ldmia r0!, {{r4-r11}}",);

        asm!("msr psp, r0", "isb",);

        asm!(        // Pop the core registers
            "bx lr",
            in("lr") 0xFFFFFFFD as usize
        );
    }
}
