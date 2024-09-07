use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::BorrowMut;
use core::ptr::addr_of;
use core::usize;

use core::result::Result;

use crate::alloc::string::ToString;
use alloc::alloc::*;
use core::mem;
use cortex_m_semihosting::hprintln;
#[allow(dead_code)]
struct ListNode {
    next: Option<usize>,
    prev: Option<usize>,
}

#[repr(C)]
pub struct TCB {
    pub top_of_stack: usize,
    pub stack_addr: usize,
    name: &'static str,
    stack_size: usize,
    // node: ListNode,
    next: Option<usize>,
    prev: Option<usize>,
    tick_to_delay: usize,
}

fn task_exit_error() {
    hprintln!("task_exit_error").unwrap();
    loop {}
}

fn init_task_stack(top_of_stack: &mut usize, func: fn(usize), p_args: usize) {
    unsafe {
        *top_of_stack -= 1 * mem::size_of::<usize>();
        let ptr = (*top_of_stack) as *mut usize;
        *ptr = 0x0100_0000;
        *top_of_stack -= 1 * mem::size_of::<usize>();
        *(*top_of_stack as *mut usize) = 0xffff_fffe & (func as usize);
        *top_of_stack -= 1 * mem::size_of::<usize>();
        *(*top_of_stack as *mut usize) = task_exit_error as usize;
        *top_of_stack -= 5 * mem::size_of::<usize>();
        *(*top_of_stack as *mut usize) = p_args;
        *top_of_stack -= 8 * mem::size_of::<usize>();
    }
    ()
}

// pub const SCB_ICSR_PENDSVSET: u32 = 1 << 28;
#[macro_export]
macro_rules! taks_yeild {
    () => {
        cortex_m::peripheral::SCB::set_pendsv();
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        // unsafe {

        //     *(0xE000_ED04 as *mut u32) =  SCB_ICSR_PENDSVSET;

        //     asm! {
        //         "dsb",
        //         "isb",
        //     };
        // }
    };
}

static mut TASK_READY_LIST: ListNode = ListNode {
    next: None,
    prev: None,
};

pub static mut TASK_VEC: Vec<TCB> = Vec::new();

#[no_mangle]
pub static mut CURRENT_TASK: Option<*const TCB> = None;
pub fn create_task(
    func: fn(usize),
    task_name: &'static str,
    size: usize,
    arg: usize,
) -> Result<(), &'static str> {
    let layout = Layout::from_size_align(size, 1).unwrap();
    let memory = unsafe { alloc(layout) };

    if memory.is_null() {
        return Err("memory.is_null");
    }

    // disable_interrupts();
    unsafe {
        let mut top_of_stack = memory as usize + (size - 1);
        top_of_stack = top_of_stack & (!(0x0007));
        let mut tcb = TCB {
            top_of_stack: top_of_stack,
            stack_addr: memory as usize,
            name: task_name,
            stack_size: size,
            prev: None,
            next: None,
            tick_to_delay: 0,
        };

        init_task_stack(&mut tcb.top_of_stack, func, arg);
        hprintln!(
            "task: start{:x} top{:x} top{:x} ",
            memory as usize,
            tcb.top_of_stack,
            memory as usize + (size - 1)
        )
        .unwrap();

        TASK_VEC.push(tcb);

        let new_item_id = TASK_VEC.len() - 1;

        if let Some(old_first_item_id) = TASK_READY_LIST.next {
            TASK_VEC[old_first_item_id].prev = Some(new_item_id);
        }

        TASK_VEC[new_item_id].next = TASK_READY_LIST.next;
        TASK_VEC[new_item_id].prev = None;

        TASK_READY_LIST.next = Some(new_item_id);
    }
    // enable_interrupts();
    Ok(())
    // 使用 `memory` 进行读写操作...

    // 使用完后释放内存
    // unsafe {
    //     dealloc(memory, layout);
    // }
}

pub fn create_static_task(
    func: fn(usize),
    task_name: &'static str,
    arg: usize,
    stack_addr: usize,
    size: usize,
    tcb: &mut TCB,
) -> Result<(), &'static str> {
    // disable_interrupts();

    let mut top_of_stack = stack_addr as usize + (size - 1);
    top_of_stack = top_of_stack & (!(0x0007));
    {
        tcb.top_of_stack = top_of_stack;
        tcb.stack_addr = stack_addr as usize;
        tcb.name = task_name;
        tcb.stack_size = size;
        tcb.prev = None;
        tcb.next = None;
        tcb.tick_to_delay = 0;
    };

    init_task_stack(&mut tcb.top_of_stack, func, arg);
    hprintln!(
        "task: start{:x} top{:x} top{:x} ",
        stack_addr as usize,
        tcb.top_of_stack,
        stack_addr as usize + (size - 1)
    )
    .unwrap();

    // enable_interrupts();
    Ok(())
    // 使用 `memory` 进行读写操作...

    // 使用完后释放内存
    // unsafe {
    //     dealloc(memory, layout);
    // }
}

fn idle_task(_arg: usize) {
    loop {
        cortex_m::asm::wfi();
    }
}
static mut IDLE_TASK_STACK: [u8; 500] = [0; 500];
static mut IDLE_TASK_TCB: TCB = TCB {
    top_of_stack: 0,
    stack_addr: 0,
    name: "idle",
    stack_size: 0,
    prev: None,
    next: None,
    tick_to_delay: 0,
};
pub fn scheduler() {
    unsafe {
        create_static_task(
            idle_task,
            "idle",
            0,
            addr_of!(IDLE_TASK_STACK) as usize,
            500,
            &mut IDLE_TASK_TCB,
        )
        .unwrap();

        if let None = CURRENT_TASK {
            TASK_READY_LIST.next.map(|item| {
                CURRENT_TASK = Some(&TASK_VEC[item]);
            });
        }
        hprintln!(
            "now task is {:x} name is {}",
            CURRENT_TASK.unwrap() as usize,
            (*(CURRENT_TASK.unwrap())).name
        )
        .unwrap();
        hprintln!("total task is {}", TASK_VEC.len()).unwrap();
        hprintln!(
            "CURRENT_TASK addr = {:x}",
            (addr_of!(CURRENT_TASK)) as *const Option<*const TCB> as usize
        )
        .unwrap();
    }
}
#[no_mangle]
pub fn task_switch_context() {
    unsafe {
        // hprintln!("old {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
        CURRENT_TASK.map(|mut _current_task| {
            if _current_task == &IDLE_TASK_TCB {
                if TASK_VEC[1].tick_to_delay == 0 {
                    CURRENT_TASK = Some(&TASK_VEC[1]);
                } else if TASK_VEC[0].tick_to_delay == 0 {
                    CURRENT_TASK = Some(&TASK_VEC[0]);
                }
            } else {
                if _current_task == &TASK_VEC[1] {
                    if TASK_VEC[0].tick_to_delay == 0 {
                        CURRENT_TASK = Some(&TASK_VEC[0]);
                    } else if TASK_VEC[0].tick_to_delay != 0 {
                        CURRENT_TASK = Some(&IDLE_TASK_TCB);
                    }
                } else if _current_task == &TASK_VEC[0] {
                    if TASK_VEC[1].tick_to_delay == 0 {
                        CURRENT_TASK = Some(&TASK_VEC[1]);
                    } else if TASK_VEC[1].tick_to_delay != 0 {
                        CURRENT_TASK = Some(&IDLE_TASK_TCB);
                    }
                }
            }
        });

        // hprintln!("switch {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
    }
}

pub fn task_delay(ms_to_delay: usize) {
    unsafe {
        CURRENT_TASK.map(|task| {
            let t = task as *mut TCB;
            (*t).tick_to_delay = ms_to_delay / 1000 * (crate::SYST_FREQ as usize);
        });
        taks_yeild!();
    }
}

pub fn systick_task_inc() {
    unsafe {
        for item in &mut TASK_VEC {
            if item.tick_to_delay > 0 {
                item.tick_to_delay -= 1;
            }
        }
    }
    taks_yeild!();
}
