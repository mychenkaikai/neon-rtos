use core::arch::global_asm;
global_asm!(
    "
    .syntax unified
    .cpu cortex-m4
    .thumb

    .global SVC_Handler
    .type SVC_Handler, %function

SVC_Handler:
    cpsid i                  @ 禁用中断
    push {{r3}}
    mrs r3, psp              @ 获取 PSP
    ldr r3, [r3, #24]        @ 从栈中加载 PC
    ldrb r3, [r3, #-2]       @ 从 SVC 指令中读取立即数
    

    bl syscall_handler 
    pop {{r3}}
    mov lr, #0xFFFFFFFD      @ 设置 LR 以使用 PSP 返回到线程模式
    cpsie i
    bx lr

    .size SVC_Handler, .-SVC_Handler
    "
);

global_asm!(
    "    
    .syntax unified
    .cpu cortex-m4
    .thumb

    .global PendSV_Handler
    .type PendSV_Handler, %function

PendSV_Handler:
    cpsid i                  @ 禁用中断

    @ 保存当前上下文
    mrs r0, psp
    stmdb r0!, {{r4-r11}}
    bl set_current_task_stack_top  @ 调用 Rust 函数保存 PSP

    @ 执行任务切换
    bl task_switch_context   @ 调用 Rust 函数进行任务切换

    @ 恢复新任务的上下文
    bl get_current_task_stack_top  @ 获取新任务的 PSP
    ldmia r0!, {{r4-r11}}
    msr psp, r0

    mov lr, #0xFFFFFFFD
    cpsie i                  @ 启用中断
    
    bx lr

    .size PendSV_Handler, .-PendSV_Handler"
);

global_asm!(
    "    
    .syntax unified
    .section .text.asm
    .global task_exit
    .global task_yield
    .global task_sleep
    

    .thumb_func
task_exit:
    PUSH {{ LR }}
    svc 0x0
    POP {{ PC }}

    .thumb_func
task_yield:
    PUSH {{ LR }}
    svc 0x1
    POP {{ PC }}

    .thumb_func
task_sleep:
    PUSH {{ LR }}
    svc 0x2
    POP {{ PC }}

    "
);
