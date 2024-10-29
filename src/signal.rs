use crate::scheduler::TCB;
extern crate alloc;
use alloc::vec::Vec;
use crate::utils::ptr::Ptr;
#[derive(PartialEq)]
pub enum BlockReason {
    Delay(usize),         // 延时阻塞，参数为剩余tick数
    Signal(SignalType), // 信号阻塞，等待特定信号
    Mutex(usize),     // 互斥量阻塞
}

#[derive(PartialEq, Copy, Clone)]
pub enum SignalType {
    Timer,            // 定时器信号
    External,         // 外部信号
    UserDefined(usize), // 用户自定义信号
}

impl From<usize> for SignalType {
    fn from(value: usize) -> Self {
        match value {
            0 => SignalType::Timer,
            1 => SignalType::External,
            _ => SignalType::UserDefined(value),
        }
    }
}

impl Into<usize> for SignalType {
    fn into(self) -> usize {
        match self {
            SignalType::Timer => 0,
            SignalType::External => 1,
            SignalType::UserDefined(value) => value,
        }
    }
}

pub struct SignalManager {
    timer_queue: Vec<Ptr<TCB>>,
    external_queue: Vec<Ptr<TCB>>,
    user_defined_queue: Vec<Ptr<TCB>>,
}

impl SignalManager {
    pub const fn new() -> Self {
        SignalManager {
            timer_queue: Vec::new(),
            external_queue: Vec::new(),
            user_defined_queue: Vec::new(),
        }
    }

    pub fn add_task_to_signal(&mut self, signal: SignalType, task: Ptr<TCB>) {
        match signal {
            SignalType::Timer => self.timer_queue.push(task),
            SignalType::External => self.external_queue.push(task),
            SignalType::UserDefined(_) => self.user_defined_queue.push(task),
        }
    }

    pub fn get_tasks_for_signal(&mut self, signal: SignalType) -> Vec<Ptr<TCB>> {
        match signal {
            SignalType::Timer => core::mem::take(&mut self.timer_queue),
            SignalType::External => core::mem::take(&mut self.external_queue),
            SignalType::UserDefined(_) => core::mem::take(&mut self.user_defined_queue),
        }
    }
}
