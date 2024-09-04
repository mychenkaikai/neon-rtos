use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::borrow::BorrowMut;
use core::fmt::Error;
use core::ptr;
use core::usize;
use lazy_static::lazy_static;
// use core::intrinsics::nearbyintf32;
use core::result::Result;
use cortex_m::interrupt::enable;
// extern crate alloc;
use crate::interrupts::*;
use crate::lock::*;
use alloc::alloc::*;
use alloc::collections::LinkedList;
use alloc::sync::Arc;
use core::mem;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_semihosting::hprintln;
use lazy_static::*;

struct ListNode {
    next: Option<usize>,
    prev: Option<usize>,
}
// impl ListNode {
//     fn set_next(&mut self,item:& Arc<Self>)
//     {
//         self.next = Some(item.clone());
//     }
//     fn set_prev(&mut self,item:&Arc<Self>)
//     {
//         self.prev = Some(item.clone());
//     }
// }
// struct TinyHeader {
//     next:Option<Box<ListNode>>,
//     prev:Option<Box<ListNode>>,
// }

#[repr(C)]
pub struct TCB {
    pub top_of_stack: usize,
    stack_addr: usize,
    name: String,
    stack_size: usize,
    // node: ListNode,
    next: Option<usize>,
    prev: Option<usize>,
}

fn task_exit_error() {
    hprintln!("task_exit_error");
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
    // hprintln!("psp:{:x} msp:{:x}",cortex_m::register::psp::read(),cortex_m::register::msp::read());
    // cortex_m::peripheral::NVIC::
    cortex_m::peripheral::SCB::set_pendsv();
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

// static spinlock: Spinlock = Spinlock::new();

// static mut TASK_BLOCK_LIST: LinkedList<TCB> = LinkedList::new();
// static mut CURRENT_TASK: Option<&TCB>  = None;

// lazy_static!{
//     static ref TASK_READY_LIST:ListNode = ListNode{
//         next:None,
//         prev:None,
//     };
// }
static mut TASK_READY_LIST: ListNode = ListNode {
    next: None,
    prev: None,
};

pub static mut TASK_VEC: Vec<TCB> = Vec::new();

pub static mut CURRENT_TASK: Option<usize> = None;

pub fn create_task(
    func: fn(usize),
    task_name: String,
    size: usize,
    arg: usize,
) -> Result<(), &'static str> {
    // let boxed_memory: Box<[u8; size]> = Box::new([0u8; size]);
    let layout = Layout::from_size_align(size, 1).unwrap();
    let memory = unsafe { alloc(layout) };

    if memory.is_null() {
        // panic!("Failed to allocate memory.");
        return Err("memory.is_null");
    }

    let mut memory_slice = unsafe { core::slice::from_raw_parts_mut(memory, size) };
    // memory_slice[11] = 9;

    // disable_interrupts();
    unsafe {
        // TASK_READY_LIST.push_back(TCB {
        //     top_of_stack: memory as usize,
        //     stack_addr: memory as usize,
        //     name: task_name,
        //     stack_size: size,
        // });
        // let b = get_l_ptr(&TASK_READY_LIST) ;
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
        hprintln!("1 {:x} {:x}", memory as usize, top_of_stack);
        // init_task_stack(&mut tcb.top_of_stack, func, arg);
        init_task_stack(&mut tcb.top_of_stack, func, arg);
        hprintln!("2 {:x} {:x}", memory as usize, tcb.top_of_stack);
        TASK_VEC.push(tcb);

        let new_item_id = TASK_VEC.len() - 1;

        if let Some(old_first_item_id) = TASK_READY_LIST.next {
            TASK_VEC[old_first_item_id].prev = Some(new_item_id);
        }

        TASK_VEC[new_item_id].next = TASK_READY_LIST.next;
        TASK_VEC[new_item_id].prev = None;

        TASK_READY_LIST.next = Some(new_item_id);

        // TASK_READY_LIST.next = Some(TASK_VEC.len());
        // let c = get_l_ptr(&a.node) ;
        // TASK_READY_LIST.next = Some(c as usize);
        // let a = 0;
        // TASK_BLOCK_LIST.extend(TASK_READY_LIST.drain(0..0));
        // TASK_BLOCK_LIST.push(TASK_READY_LIST.swap_remove(0));
        // hprintln!("{} {}", TASK_READY_LIST.len(), TASK_BLOCK_LIST.len());
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
                CURRENT_TASK = Some(item);
            }
        }

        hprintln!(
            "now task is {} name is {}",
            CURRENT_TASK.unwrap(),
            TASK_VEC[CURRENT_TASK.unwrap()].name
        );
        hprintln!("total task is {}", TASK_VEC.len());
    }
}

pub fn task_switch_context() {
    unsafe {
        if let Some(mut id) = CURRENT_TASK {
            if id == 1 {
                id = 0;
            } else {
                id = 1;
            }
            CURRENT_TASK = Some(id);
        }
       
        // hprintln!("switch {}",CURRENT_TASK.unwrap());
    }
    
}
