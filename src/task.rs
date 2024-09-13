use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ptr::addr_of;
use core::usize;

use core::result::Result;

use alloc::alloc::*;
use alloc::collections::LinkedList;

use core::mem;
use cortex_m_semihosting::hprintln;
#[derive(PartialEq)]
#[repr(C)]
pub struct TCB {
    pub top_of_stack: usize,
    pub stack_addr: usize,
    name: &'static str,
    stack_size: usize,
    next: Option<usize>,
    prev: Option<usize>,
    item_value: Option<usize>,
    id: usize,
}

impl TCB {
    fn new() -> Self {
        Self {
            top_of_stack: 0,
            stack_addr: 0,
            name: "none",
            stack_size: 0,
            prev: None,
            next: None,
            item_value: None,
            id: 0,
        }
    }

    unsafe fn next<'a>(&self) -> Option<&'a TCB> {
        self.next.map_or(None, |id| Some(TASK_TABLE.at(id)))
    }
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

static mut TASK_TABLE: TaskTable = TaskTable::new();
static mut TASK_READY_LIST: List = List::new();

static mut TASK_DELAY_LIST: List = List::new();

static mut TASK_NEW_READY_LIST: LinkedList<TCB> = LinkedList::new();

#[no_mangle]
pub static mut CURRENT_NEW_TASK: Option<&TCB> = None;

#[no_mangle]
pub static mut CURRENT_NEW_NEW_TASK: Option<&TCB> = None;

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
        // tcb.id = self.task_vec.len();
        self.task_vec.push(tcb);
    }

    // fn get_id(&mut self, tcb: &TCB) -> usize {
    //     if let Some(prev) = tcb.prev {
    //         self.at(prev).next.unwrap()
    //     } else if let Some(next) = tcb.next {
    //         self.at(next).prev.unwrap()
    //     } else {
    //         for (i, item) in self.task_vec.iter().enumerate() {
    //             if *item == *tcb {
    //                 return i;
    //             }
    //         }
    //         panic!()
    //     }
    // }
}

struct List {
    next: Option<usize>,
    prev: Option<usize>,
    len: usize,
}
impl List {
    const fn new() -> Self {
        Self {
            next: None,
            prev: None,
            len: 0,
        }
    }

    unsafe fn get_first<'a>(&mut self) -> Option<&'a TCB> {
        if self.len > 0 {
            return Some(TASK_TABLE.at(self.next.unwrap()));
        }
        None
    }

    unsafe fn get_first_mut<'a>(&mut self) -> Option<&'a mut TCB> {
        if self.len > 0 {
            return Some(TASK_TABLE.at_mut(self.next.unwrap()));
        }
        None
    }

    // fn get_first_id<'a>(&mut self) -> Option<usize> {
    //     if self.len > 0 {
    //         return Some(self.next.unwrap());
    //     }
    //     None
    // }

    unsafe fn ins_to_first(&mut self, new_item_id: usize) {
        if self.len > 0 {
            let first_item = TASK_TABLE.at_mut(self.next.unwrap());
            first_item.prev = Some(new_item_id);

            let last_item = TASK_TABLE.at_mut(self.prev.unwrap());
            last_item.next = Some(new_item_id);

            let tcb = TASK_TABLE.at_mut(new_item_id);
            tcb.next = self.next;
            tcb.prev = self.prev;
        } else {
            let tcb = TASK_TABLE.at_mut(new_item_id);
            tcb.next = Some(new_item_id);
            tcb.prev = Some(new_item_id);
        }

        self.next = Some(new_item_id);
        self.prev = Some(new_item_id);
        self.len += 1;
    }

    unsafe fn del(&mut self, del_item_id: usize) {
        if self.len == 0 {
            panic!()
        } else if self.len == 1 {
            self.next = None;
            self.prev = None;
            self.len = 0;
        } else {
            let tcb = TASK_TABLE.at_mut(del_item_id);
            let prev_item = TASK_TABLE.at_mut(tcb.prev.unwrap());

            prev_item.next = tcb.next;

            let next_item = TASK_TABLE.at_mut(tcb.next.unwrap());
            next_item.prev = tcb.prev;
            tcb.next = None;
            tcb.prev = None;
        }
    }

    unsafe fn sort(&mut self) {
        if self.len <= 1 {
            return;
        } else {
            let mut last = self.get_first_mut().unwrap();
            for (i, item) in self.into_iter().enumerate() {
                if i > 0 {
                    if last.item_value > item.item_value {
                        last.next = item.next;
                        last.prev = Some(item.id);
                        item.prev = last.prev;
                        item.next = Some(last.id);
                    }
                    last = item;
                }
            }
        }
    }

    unsafe fn move_task_to_another_list(src: &mut List, dst: &mut List, tcb_id: usize) {
        if src.len == 0 {
            panic!()
        } else if src.len == 1 {
            src.next = None;
            src.prev = None;
            src.len = 0;
        } else {
            let tcb_next = TASK_TABLE.at(tcb_id).next.unwrap();
            let tcb_prev = TASK_TABLE.at(tcb_id).prev.unwrap();

            let prev_item = TASK_TABLE.at_mut(tcb_prev);

            prev_item.next = Some(tcb_next);
            let next_item = TASK_TABLE.at_mut(tcb_next);
            next_item.prev = Some(tcb_prev);
        }

        {
            let new_list_item_id = tcb_id;
            if dst.len > 0 {
                let first_list_item = TASK_TABLE.at_mut(dst.next.unwrap());
                first_list_item.prev = Some(new_list_item_id);

                let last_list_item = TASK_TABLE.at_mut(dst.prev.unwrap());
                last_list_item.next = Some(new_list_item_id);

                TASK_TABLE.at_mut(tcb_id).next = dst.next;
                TASK_TABLE.at_mut(tcb_id).prev = dst.prev;
            } else {
                TASK_TABLE.at_mut(tcb_id).next = Some(new_list_item_id);
                TASK_TABLE.at_mut(tcb_id).prev = Some(new_list_item_id);
            }
            dst.next = Some(new_list_item_id);
            dst.prev = Some(new_list_item_id);
            dst.len += 1;
        }
    }
}
impl<'a> IntoIterator for &'a List {
    type IntoIter = TaskListIter<'a>;
    type Item = &'a TCB;
    fn into_iter(self) -> Self::IntoIter {
        TaskListIter {
            len: self.len,
            item_id: match self.next {
                Some(item_id) => item_id,
                _ => 0,
            },

            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> IntoIterator for &'a mut List {
    type IntoIter = TaskListIterMut<'a>;
    type Item = &'a mut TCB;

    fn into_iter(self) -> Self::IntoIter {
        TaskListIterMut {
            len: self.len,
            item_id: match self.next {
                Some(item_id) => item_id,
                _ => 0,
            },
            index: 0,
            _marker: PhantomData,
        }
    }
}

struct TaskListIter<'a> {
    item_id: usize,
    // TASK_TABLE: &'a [TCB],
    // task_table: &'a TaskTable,
    index: usize,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for TaskListIter<'a> {
    type Item = &'a TCB;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        if self.index >= self.len {
            return None;
        }
        self.index += 1;
        unsafe {
            let a = TASK_TABLE.at(self.item_id);
            Some(a)
        }
    }
}

struct TaskListIterMut<'a> {
    item_id: usize,
    // TASK_TABLE: &'a mut TaskTable,
    index: usize,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for TaskListIterMut<'a> {
    type Item = &'a mut TCB;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        if self.index >= self.len {
            return None;
        }
        self.index += 1;
        unsafe {
            let a = TASK_TABLE.at_mut(self.item_id);
            Some(a)
        }
    }
}

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
        let mut tcb = TCB::new();

        create_static_task(func, task_name, arg, memory as usize, size, &mut tcb);
        let new_id = TASK_TABLE.len();
        tcb.id = new_id;
        TASK_TABLE.add(tcb);

        TASK_READY_LIST.ins_to_first(new_id);
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

    tcb.top_of_stack = top_of_stack;
    tcb.stack_addr = stack_addr as usize;
    tcb.name = task_name;
    tcb.stack_size = size;
    tcb.prev = None;
    tcb.next = None;
    tcb.item_value = None;

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
    item_value: None,
    id: 0,
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

        if let None = CURRENT_NEW_TASK {
            TASK_READY_LIST.next.map(|item| {
                CURRENT_NEW_TASK = Some(TASK_TABLE.at(item));
            });
        }
        hprintln!("now task is name is {}", CURRENT_NEW_TASK.unwrap().name).unwrap();
        hprintln!("total task is {}", TASK_TABLE.len()).unwrap();
    }
}
#[no_mangle]
pub fn task_switch_context() {
    unsafe {
        // hprintln!("old {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
        CURRENT_NEW_TASK.map(|_current_task| {
            if _current_task == &IDLE_TASK_TCB {
                if TASK_READY_LIST.len > 0 {
                    CURRENT_NEW_TASK = TASK_READY_LIST.get_first();
                }
            } else {
                if let Some(next_tcb) = _current_task.next() {
                    CURRENT_NEW_TASK = Some(next_tcb);
                }
            }
        });

        // hprintln!("switch {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
    }
}
static mut next_delay_task_unblock_time: Option<usize> = None;
use cortex_m::peripheral::SYST;
pub fn task_delay(ms_to_delay: usize) {
    unsafe {
        CURRENT_NEW_TASK.map(|task| {
            let tick_to_delay = ms_to_delay / 1000 * (crate::SYST_FREQ as usize);

            let current_tick = SYST::get_current() as usize;

            let expect_tick = current_tick + tick_to_delay;
            TASK_TABLE.at_mut(task.id).item_value = Some(expect_tick);

            if expect_tick < current_tick {
            } else {
                next_delay_task_unblock_time = Some(expect_tick);
                List::move_task_to_another_list(
                    &mut TASK_READY_LIST,
                    &mut TASK_DELAY_LIST,
                    task.id,
                );
            }
        });
        taks_yeild!();
    }
}

pub fn systick_task_inc() {
    let mut should_switch = false;
    unsafe {
        if let Some(expect_time) = next_delay_task_unblock_time {
            let current_time = SYST::get_current() as usize;
            if current_time >= expect_time {
                loop {
                    if TASK_DELAY_LIST.len == 0 {
                        next_delay_task_unblock_time = None;
                        break;
                    } else {
                        let item = TASK_DELAY_LIST.get_first().unwrap();

                        if item.item_value.unwrap() > current_time {
                            break;
                        }

                        List::move_task_to_another_list(
                            &mut TASK_DELAY_LIST,
                            &mut TASK_READY_LIST,
                            item.id,
                        );

                        // TASK_DELAY_LIST.sort(&mut TASK_TABLE);
                    }
                }
            }
        }

        if (TASK_READY_LIST.len > 0) {
            should_switch = true;
        }
    }
    if should_switch {
        taks_yeild!();
    }
}
