// 导入内联汇编宏
// extern crate cortex_m;
// use cortex_m::asm;
use core::arch::{asm, global_asm};
use cortex_m_semihosting::{debug, hprintln};
/// 启动第一个任务
pub unsafe fn vPortStartFirstTask() {
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
//            "ldr r3, =tmp",
pub extern "C" fn vPortSVCHandler() {
    unsafe {
        let tmp = crate::task::TASK_VEC[crate::task::CURRENT_TASK.unwrap()].top_of_stack;
        let mut var:usize;
        // asm!(
        //     ".align 4",
        //     "ldr r0, [r1]",
        //     in("r1") tmp,
        //     lateout("r0") var
        // );
        // hprintln!("var:{:x}",var);
        asm!(
            ".align 4",
            "ldmia r0!, {{r4-r11}}",          // Pop the core registers
            in("r0") tmp,
            lateout("r0") var,
            // options(nostack)
        );
        // hprintln!("var:{:x}",var);
        asm!(
            "msr psp, r0",        // Pop the core registers
            "mrs r0, psp", 
            lateout("r0") var
        );
        // hprintln!("var:{:x}",var);
        asm!(        // Pop the core registers
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

global_asm!(include_str!("port.s"));

extern "C"{
    pub fn vPortPenSVHandler();
} 
