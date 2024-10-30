
use core::arch::global_asm;


global_asm!(
    "    
    .syntax unified
    .section .text.asm
    .global call_task_exit
    .global call_task_yield
    .global call_task_sleep

    .global call_task_wait_signal
    .global call_task_send_signal
    .global call_task_mutex_lock
    .global call_task_mutex_unlock

    .thumb_func
call_task_exit:
    PUSH {{ LR }}
    svc 0x0
    POP {{ PC }}

    .thumb_func
call_task_yield:
    PUSH {{ LR }}
    svc 0x1
    POP {{ PC }}

    .thumb_func
call_task_sleep:
    PUSH {{ LR }}
    svc 0x2
    POP {{ PC }}

    .thumb_func
call_task_wait_signal:
    PUSH {{ LR }}
    svc 0x3
    POP {{ PC }}

    .thumb_func
call_task_send_signal:
    PUSH {{ LR }}
    svc 0x4
    POP {{ PC }}

    .thumb_func
call_task_mutex_lock:
    PUSH {{ LR }}
    svc 0x5
    POP {{ PC }}

    .thumb_func
call_task_mutex_unlock:
    PUSH {{ LR }}
    svc 0x6
    POP {{ PC }}
    "
);
