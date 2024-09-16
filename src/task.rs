use alloc::vec::Vec;

use core::ptr::{addr_of, NonNull};
use core::usize;
use cortex_m::interrupt::free;

use core::result::Result;

use crate::list::*;
use crate::mem::mem_alloc;
use crate::taks_yeild;
use crate::task::LinkType::*;
use core::mem;
use cortex_m_semihosting::hprintln;

use cortex_m::peripheral::SYST;

#[derive(PartialEq)]
#[repr(C)]
pub struct TCB {
    pub top_of_stack: usize,
    pub stack_addr: usize,
    name: &'static str,
    stack_size: usize,
    // next: Option<LinkType>,
    // prev: Option<LinkType>,
    // item_value: Option<usize>,
    // id: TcbId,
    pub link_node: LinkNode,
}

impl TCB {
    fn new() -> Self {
        Self {
            top_of_stack: 0,
            stack_addr: 0,
            name: "none",
            stack_size: 0,
            link_node: LinkNode {
                prev: LinkType::Null,
                next: LinkType::Null,
                item_value: None,
                id: LinkType::Null,
            },
        }
    }

    // unsafe fn next<'a>(&self) -> Option<&'a TCB> {
    //     match self.next{
    //         LinkType::TcbId(_) => todo!(),
    //         LinkType::None => todo!(),
    //         LinkType::ListId(_) => todo!(),
    //     }
    //     // self.next.map_or(None, |id| Some(TASK_TABLE.at(id)))
    // }
}

fn task_exit_error() {
    hprintln!("task_exit_error").unwrap();
    loop {}
}

fn init_task_stack(top_of_stack: &mut usize, func: fn(usize), p_args: usize) {
    // ptr::read_volatile(0x2FFF_FFFF as *const u32);
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

static mut NEXT_DELAY_TASK_UNBLOCK_TIME: Option<usize> = None;
static mut TASK_TABLE: TaskTable = TaskTable::new();

#[no_mangle]
pub static mut CURRENT_TASK: Option<&TCB> = None;

struct TaskTable {
    task_vec: Vec<TCB>,
}

impl TaskTable {
    const fn new() -> Self {
        Self {
            task_vec: Vec::new(),
        }
    }

    fn at<'a>(&'a self, id: usize) -> &'a TCB {
        &self.task_vec[id]
    }

    fn at_mut<'a>(&'a mut self, id: usize) -> &'a mut TCB {
        &mut self.task_vec[id]
    }

    fn len(&mut self) -> usize {
        self.task_vec.len()
    }

    fn add(&mut self, tcb: TCB) {
        self.task_vec.push(tcb);
    }
}

pub fn create_task(
    func: fn(usize),
    task_name: &'static str,
    size: usize,
    arg: usize,
) -> Result<(), &'static str> {
    // disable_interrupts();
    unsafe {
        let mut tcb = TCB::new();
        let memory = mem_alloc(size);
        create_static_task(func, task_name, arg, memory as usize, size, &mut tcb);
        let new_id = TASK_TABLE.len();
        tcb.link_node.id = Tcb(TcbId(new_id));
        TASK_TABLE.add(tcb);

        TASK_READY_LIST.ins_to_first(TcbId(new_id));
    }
    // enable_interrupts();
    Ok(())
    // 使用 `memory` 进行读写操作...

    // 使用完后释放内存
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

    tcb.top_of_stack = top_of_stack;
    tcb.stack_addr = stack_addr as usize;
    tcb.name = task_name;
    tcb.stack_size = size;
    tcb.link_node.prev = Null;
    tcb.link_node.next = Null;
    tcb.link_node.item_value = None;

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
    link_node: LinkNode {
        prev: Null,
        next: Null,
        item_value: None,
        id: Null,
    },
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
            TASK_READY_LIST.get_first().map(|item| {
                CURRENT_TASK = Some(item);
            });
        }
        hprintln!("now task is name is {}", CURRENT_TASK.unwrap().name).unwrap();
        hprintln!("total task is {}", TASK_TABLE.len()).unwrap();
    }
}
#[no_mangle]
pub fn task_switch_context() {
    unsafe {
        // hprintln!("old {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
        CURRENT_TASK.map(|_current_task| {
            if _current_task == &IDLE_TASK_TCB {
                if TASK_READY_LIST.len > 0 {
                    CURRENT_TASK = TASK_READY_LIST.get_first();
                }
            } else {
                match _current_task.link_node.next {
                    Tcb(tcb) => CURRENT_TASK = Some(tcb.get()),
                    LinkType::Listid(list) => CURRENT_TASK = list.get_mut().get_first(),
                    Null => panic!(),
                }
            }
        });

        // hprintln!("switch {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
    }
}

pub fn task_delay(ms_to_delay: usize) {
    unsafe {
        CURRENT_TASK.map(|task| {
            let tick_to_delay = ms_to_delay / 1000 * (crate::SYST_FREQ as usize);

            let current_tick = SYST::get_current() as usize;

            let expect_tick = current_tick + tick_to_delay;
            task.link_node.id.get_mut().item_value = Some(expect_tick);

            if expect_tick < current_tick {
            } else {
                NEXT_DELAY_TASK_UNBLOCK_TIME = Some(expect_tick);
                if let Tcb(tcb_id) = task.link_node.id {
                    TASK_READY_LIST.del(tcb_id);
                    TASK_DELAY_LIST.ins_to_first(tcb_id);
                }
            }
        });
        taks_yeild!();
    }
}

pub fn systick_task_inc() {
    let mut should_switch = false;
    unsafe {
        if let Some(expect_time) = NEXT_DELAY_TASK_UNBLOCK_TIME {
            let current_time = SYST::get_current() as usize;
            if current_time >= expect_time {
                loop {
                    if TASK_DELAY_LIST.len == 0 {
                        NEXT_DELAY_TASK_UNBLOCK_TIME = None;
                        break;
                    } else {
                        let item = TASK_DELAY_LIST.get_first().unwrap().link_node;

                        if item.item_value.unwrap() > current_time {
                            break;
                        }

                        if let Tcb(tcb_id) = item.id {
                            TASK_READY_LIST.del(tcb_id);
                            TASK_DELAY_LIST.ins_to_first(tcb_id);
                        }

                        // TASK_DELAY_LIST.sort(&mut TASK_TABLE);
                    }
                }
            }
        }

        if TASK_READY_LIST.len > 0 {
            should_switch = true;
        }
    }
    if should_switch {
        taks_yeild!();
    }
}
#[derive(PartialEq, Clone, Copy)]
pub struct TcbId(usize);
impl TcbId {
    fn new(id: usize) -> Self {
        Self(id)
    }
    pub fn get<'a>(&self) -> &'a TCB {
        unsafe { TASK_TABLE.at(self.0) }
    }

    pub fn get_mut<'a>(&self) -> &'a mut TCB {
        unsafe { TASK_TABLE.at_mut(self.0) }
    }
}
#[derive(PartialEq, Clone, Copy)]
pub struct ListId(usize);

impl ListId {
    fn get_mut(&self) -> &mut crate::task::List {
        if self.0 == 0 {
            unsafe { &mut TASK_READY_LIST }
        } else {
            unsafe { &mut TASK_READY_LIST }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct LinkNode {
    pub next: LinkType,
    pub prev: LinkType,
    item_value: Option<usize>,
    pub id: LinkType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LinkType {
    Tcb(TcbId),
    Listid(ListId),
    Null,
}

impl LinkNode {
    pub const fn new() -> Self {
        Self {
            next: Listid(ListId(0)),
            prev: Listid(ListId(0)),
            item_value: None,
            id: Listid(ListId(0)),
        }
    }
    pub fn join(&mut self, second_node: &mut LinkNode) {
        let third_node = self.next.get_mut();

        third_node.prev = second_node.id;
        second_node.next = third_node.id;
        self.next = second_node.id;
        second_node.prev = self.id;

        // let second = second_node.get_mut();
    }
    pub fn del(&mut self) {
        let mut a = self.prev.get_mut();
        let mut b = self.next.get_mut();
        a.join(b);
    }
}

impl LinkType {
    fn get_mut(&self) -> &mut LinkNode {
        match self {
            Tcb(id) => &mut id.get_mut().link_node,
            Listid(id) => &mut id.get_mut().link_node,
            Null => panic!(),
        }
    }
}
