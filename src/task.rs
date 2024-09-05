use alloc::string::String;
use alloc::vec::Vec;
use core::usize;

use core::result::Result;

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
    name: String,
    stack_size: usize,
    // node: ListNode,
    next: Option<usize>,
    prev: Option<usize>,
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

pub fn taks_yeild() {
    cortex_m::peripheral::SCB::set_pendsv();
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

static mut TASK_READY_LIST: ListNode = ListNode {
    next: None,
    prev: None,
};

pub static mut TASK_VEC: Vec<TCB> = Vec::new();

pub static mut CURRENT_TASK: Option<*const TCB> = None;
pub fn create_task(
    func: fn(usize),
    task_name: String,
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

pub fn scheduler() {
    unsafe {
        if let None = CURRENT_TASK {
            if let Some(item) = TASK_READY_LIST.next {
                CURRENT_TASK = Some(&TASK_VEC[item]);
            }
        }
        hprintln!(
            "now task is {} name is {}",
            CURRENT_TASK.unwrap() as usize,
            (*(CURRENT_TASK.unwrap())).name
        )
        .unwrap();
        hprintln!("total task is {}", TASK_VEC.len()).unwrap();
        hprintln!(
            "CURRENT_TASK addr = {:x}",
            (&(CURRENT_TASK.unwrap())) as *const *const TCB as usize
        )
        .unwrap();
    }
}
#[no_mangle]
pub fn task_switch_context() {
    unsafe {
        hprintln!("old {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
        if let Some(mut id) = CURRENT_TASK {
            if id == &TASK_VEC[0] {
                id = &TASK_VEC[1];
            } else {
                id = &TASK_VEC[0];
            }
            CURRENT_TASK = Some(id);
        }

        hprintln!("switch {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
    }
}
