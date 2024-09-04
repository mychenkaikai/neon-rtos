xPortPendSVHandler:
    mrs r0, psp
    isb
    ldr r3, =pxCurrentTCB
    ldr r2, [r3]

    stmdb r0!, {{r4-r11}}
    str r0, [r2]

    stmdb sp!, {{r3, r14}}
    /* configMAX_SYSCALL_INTERRUPT_PRIORITY*/
    mov r0, #0
    msr basepri, r0
    dsb
    isb
    bl vTaskSwitchContext
    mov r0, #0
    msr basepri, r0
    ldmia sp!, {{r3, r14}}

    ldr r1, [r3]
    ldr r0, [r1]                    /* The first item in pxCurrentTCB is the task top of stack. */
    ldmia r0!, {{r4-r11}}           /* Pop the registers. */
    msr psp, r0
    isb
    bx r14