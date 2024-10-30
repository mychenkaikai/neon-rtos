use crate::arch::common::ArchPortTrait;
use crate::arch::common::MemOperations;
use crate::arch::port::mem::ArchMem;
use crate::arch::port::ArchPort;
use crate::kernel::scheduler::with_scheduler;
use crate::kernel::sync::signal::*;

use crate::kernel_println;
use crate::utils::double_list::{Linkable, NodePtr};
// pub fn create_task(
//     name: &'static str,
//     stack_size: usize,
//     entry: fn(usize),
// ) -> Result<(), &'static str> {
//     with_scheduler(|s| s.create_task(name, stack_size, entry))
// }

pub fn create_task(
    name: &'static str,
    stack_size: usize,
    entry: fn(usize),
) -> Result<(), &'static str> {
    with_scheduler(|s| {
        ArchPort::disable_interrupts();
        let tcb = TCB::new(name, stack_size, entry);

        let node = s.task_ready_list.push_back(tcb);
        kernel_println!(
            "Task added to ready list. List size: {}",
            s.task_ready_list.len()
        );

        if let Some(mut element) = node.data {
            element.set_node_ptr(Some(node));
            kernel_println!("Node pointer set for task: {}", name);
        } else {
            kernel_println!("Failed to set node pointer for task: {}", name);
            return Err("Failed to set node pointer");
        }

        kernel_println!("Task '{}' created successfully", name);
        ArchPort::enable_interrupts();
        Ok(())
    })
}

pub fn start(entry: fn(), ticks_per_second: usize) -> ! {
    with_scheduler(|s| s.start(entry, ticks_per_second))
}

#[derive(PartialEq)]
pub enum BlockReason {
    Delay(usize),       // 延时阻塞，参数为剩余tick数
    Signal(SignalType), // 信号阻塞，等待特定信号
    Mutex(usize),       // 互斥量阻塞
}

extern crate alloc;
use alloc::vec::Vec;
#[derive(PartialEq)]
pub(crate) enum TaskState {
    Ready,
    Running,
    Blocked(BlockReason),
}
// #[derive(Debug)]
#[repr(C)]
pub struct TCB {
    // 任务控制块的字段
    pub(crate) stack_top: usize,
    pub(crate) name: &'static str,
    pub(crate) stack_addr: usize,
    pub(crate) stack_size: usize,
    pub(crate) node_ptr: Option<NodePtr<Self>>,
    pub(crate) unblock_time: Option<usize>,
    pub(crate) state: TaskState,
    pub(crate) pending_signals: Vec<SignalType>, // 待处理的信号
    pub(crate) waiting_signals: Vec<SignalType>, // 等待的信号
}
const STACK_CANARY: u32 = 0xDEADBEEF;
impl TCB {
    pub fn new(name: &'static str, stack_size: usize, entry: fn(usize)) -> Self {
        // 这里需要实现实际的栈分配和初始化逻辑
        let stack_addr: usize = ArchMem::mem_alloc(stack_size) as usize;
        unsafe {
            *(stack_addr as *mut u32) = STACK_CANARY;
        }
        let mut stack_top = stack_addr as usize + (stack_size - 1);
        stack_top = stack_top & (!(0x0007));

        let mut tcb = Self {
            name,
            stack_top,
            stack_addr,
            stack_size,
            node_ptr: None,
            unblock_time: None,
            state: TaskState::Ready,
            pending_signals: Vec::new(),
            waiting_signals: Vec::new(),
        };
        kernel_println!("stack_top: {:x}", tcb.stack_top);
        kernel_println!("stack_addr: {:x}", tcb.stack_addr);
        kernel_println!("stack_size: {:x}", tcb.stack_size);
        ArchPort::init_task_stack(&mut tcb.stack_top, entry, 0);
        tcb
    }
    pub(crate) fn check_stack_overflow(&self) -> bool {
        unsafe { *(self.stack_addr as *const u32) != STACK_CANARY }
    }
    pub(crate) fn unblock_time(&self) -> Option<usize> {
        self.unblock_time
    }

    pub(crate) fn set_unblock_time(&mut self, time: Option<usize>) {
        self.unblock_time = time;
    }
}

impl Linkable for TCB {
    fn get_node_ptr(&self) -> Option<NodePtr<Self>> {
        self.node_ptr
    }

    fn set_node_ptr(&mut self, ptr: Option<NodePtr<Self>>) {
        self.node_ptr = ptr;
    }
}

pub fn idle_task(_: usize) {
    loop {
        // 空闲任务的实现
        crate::arch::port::ArchPort::idle_task();
    }
}
