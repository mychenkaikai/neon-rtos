use crate::arch::common::ArchPortTrait;
use crate::arch::common::MemOperations;

use crate::arch::port::mem::ArchMem;

use crate::arch::port::set_psp;
use crate::arch::port::ArchPort;
// use crate::utils::print::*;
use crate::kernel_println;
use crate::utils::double_list::ElementPtr;
use crate::utils::double_list::NodePtr;
use crate::utils::double_list::*;

use crate::kernel::sync::mutex::MutexManager;
use crate::kernel::sync::signal::SignalManager;
use core::ops::FnOnce;
use core::option::Option;
use core::option::Option::*;
use core::result::Result;
use core::result::Result::*;

use crate::kernel::sync::signal::*;
use crate::kernel::task::tcb::*;
pub struct Scheduler {
    pub(crate) task_ready_list: LinkList<TCB>,
    pub(crate) task_delay_list: LinkList<TCB>,
    pub(crate) current_task: Option<ElementPtr<TCB>>,
    pub(crate) task_blocked_list: LinkList<TCB>,
    pub(crate) idle_task: Option<ElementPtr<TCB>>,
    next_delay_task_unblock_time: Option<usize>,
    ticks_count: usize,
    ticks_per_second: usize,
    signal_manager: SignalManager,
    pub(crate) mutex_manager: MutexManager,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            task_ready_list: LinkList::new(),
            task_delay_list: LinkList::new(),
            task_blocked_list: LinkList::new(),
            current_task: None,
            idle_task: None,
            next_delay_task_unblock_time: None,
            ticks_count: 0,
            ticks_per_second: 100,
            signal_manager: SignalManager::new(),
            mutex_manager: MutexManager::new(),
        }
    }

    pub fn task_ready_list(&mut self) -> &mut LinkList<TCB> {
        &mut self.task_ready_list
    }

    pub fn task_delay_list(&mut self) -> &mut LinkList<TCB> {
        &mut self.task_delay_list
    }

    pub fn current_task(&self) -> Option<ElementPtr<TCB>> {
        self.current_task
    }

    pub fn set_current_task(&mut self, task: Option<ElementPtr<TCB>>) {
        self.current_task = task;
    }

    pub fn idle_task(&self) -> Option<ElementPtr<TCB>> {
        self.idle_task
    }

    pub fn set_idle_task(&mut self, task: Option<ElementPtr<TCB>>) {
        self.idle_task = task;
    }

    pub fn next_delay_task_unblock_time(&self) -> Option<usize> {
        self.next_delay_task_unblock_time
    }

    pub fn set_next_delay_task_unblock_time(&mut self, time: Option<usize>) {
        self.next_delay_task_unblock_time = time;
    }

    pub fn ticks_count(&self) -> usize {
        self.ticks_count
    }

    pub fn increment_ticks_count(&mut self) {
        self.ticks_count += 1;
    }

    pub fn start(&mut self, entry: fn(), ticks_per_second: usize) -> ! {
        // 初始化空闲任务
        self.idle_task = Some(ElementPtr::new(TCB::new("idle", 500, idle_task)));

        // 选择第一个任务开始执行
        self.current_task = self
            .task_ready_list
            .front()
            .and_then(|tcb| tcb.get_node_ptr())
            .and_then(|node| node.data)
            .or_else(|| self.idle_task);

        self.current_task
            .and_then(|tcb| Some(set_psp(tcb.stack_top + 8 * size_of::<usize>())));

        self.ticks_per_second = ticks_per_second;

        entry();

        loop {}
    }

    pub fn task_switch_context(&mut self) {
        if let Some(mut current) = self.current_task {
            // 1. 根据当前任务的状态，将其移动到对应列表
            if let Some(node) = current.get_node_ptr() {
                match current.state {
                    TaskState::Ready => {
                        // 如果任务仍处于就绪状态，保持在 ready_list
                        // 如果不在 ready_list 中，重新加入
                        // if !self.task_ready_list.contains(node) {
                        //     self.task_ready_list.attach_back(node);
                        // }
                    }
                    TaskState::Blocked(BlockReason::Signal(SignalType::Timer)) => {
                        // 如果是延时阻塞，移到延时列表
                        self.task_ready_list.detach(node);
                        self.task_delay_list.attach_back(node);
                        self.update_next_delay_task_unblock_time();
                    }
                    TaskState::Blocked(BlockReason::Mutex(_)) => {
                        // 如果是互斥锁阻塞，移到阻塞列表
                        self.task_ready_list.detach(node);
                        self.task_blocked_list.attach_back(node);
                    }
                    TaskState::Blocked(BlockReason::Signal(_)) => {
                        // 如果是信号阻塞，移到阻塞列表
                        self.task_ready_list.detach(node);
                        self.task_blocked_list.attach_back(node);
                    }
                    TaskState::Blocked(BlockReason::Delay(_)) => {
                        // 如果是延时阻塞，移到延时列表
                        self.task_ready_list.detach(node);
                        self.task_delay_list.attach_back(node);
                        self.update_next_delay_task_unblock_time();
                    }
                    TaskState::Running => {
                        // 如果任务仍在运行，将其状态改为就绪
                        current.state = TaskState::Ready;
                    }
                }
            }

            // 2. 从就绪列表中选择下一个任务
            self.current_task = self
                .task_ready_list
                .front()
                .and_then(|tcb| tcb.get_node_ptr())
                .and_then(|node| node.data)
                .or(self.idle_task);

            // 3. 设置新任务的状态为运行
            if let Some(mut next_task) = self.current_task {
                next_task.state = TaskState::Running;
            }

            // 4. 检查栈溢出
            if let Some(tcb) = self.current_task {
                if tcb.check_stack_overflow() {
                    panic!("Stack overflow detected in task: {:x}", tcb.stack_addr);
                }
            }
        }
    }

    pub fn delay_task(&mut self, ms: usize) {
        if let Some(mut current) = self.current_task {
            let ticks = (ms * self.ticks_per_second) / 1000;
            let unblock_time = self.ticks_count + ticks;
            current.set_unblock_time(Some(unblock_time));
            // 只设置状态，实际的列表迁移在 task_switch_context 中完成
            current.state = TaskState::Blocked(BlockReason::Signal(SignalType::Timer));
        }
        ArchPort::call_task_yield();
    }

    pub fn tick(&mut self) {
        ArchPort::enter_critical_section();
        self.increment_ticks_count();
        // kernel_println!("Tick: {}", self.ticks_count);
        // 检查延迟任务
        if let Some(unblock_time) = self.next_delay_task_unblock_time {
            if self.ticks_count >= unblock_time {
                self.unblock_tasks();
            }
        }
        ArchPort::exit_critical_section();

        // 如果就绪列表不为空，进行任务切换
        if !self.task_ready_list.is_empty() {
            ArchPort::call_task_yield();
        }
    }

    fn unblock_tasks(&mut self) {
        let mut next_unblock_time: Option<usize> = None;
        let current_ticks = self.ticks_count;

        let mut current = self.task_delay_list.head;
        while let Some(mut node) = current {
            let tcb = node.data.as_ref().unwrap();
            if let Some(unblock_time) = tcb.unblock_time() {
                if current_ticks >= unblock_time {
                    let next = node.next;

                    // 从延迟列表中分离节点
                    self.task_delay_list.detach(node);

                    // 重置解除阻塞时间
                    if let Some(tcb) = node.data.as_mut() {
                        tcb.set_unblock_time(None);
                        tcb.state = TaskState::Ready;
                    }

                    // 将节点添加到就绪列表
                    self.task_ready_list.attach_back(node);

                    current = next;
                } else {
                    next_unblock_time =
                        Some(next_unblock_time.map_or(unblock_time, |t| t.min(unblock_time)));
                    current = node.next;
                }
            } else {
                current = node.next;
            }
        }

        self.next_delay_task_unblock_time = next_unblock_time;
    }

    fn update_next_delay_task_unblock_time(&mut self) {
        self.next_delay_task_unblock_time = self
            .task_delay_list
            .iter()
            .filter_map(|tcb| tcb.unblock_time())
            .min();
    }
    pub fn block_task_with_signal(&mut self, signal: SignalType) {
        if let Some(mut current) = self.current_task {
            // 只设置状态，实际的列表迁移在 task_switch_context 中完成
            current.state = TaskState::Blocked(BlockReason::Signal(signal));
            self.signal_manager.add_task_to_signal(signal, current);
        }
        ArchPort::call_task_yield();
    }

    pub fn send_signal(&mut self, signal: SignalType) {
        // 直接从信号管理器获取并唤醒所有等待该信号的任务
        let tasks = self.signal_manager.get_tasks_for_signal(signal);
        for mut task in tasks {
            task.state = TaskState::Ready;
            if let Some(node) = task.get_node_ptr() {
                self.task_blocked_list.detach(node);
                self.task_ready_list.attach_back(node);
            }
        }
    }
}

pub static mut SCHEDULER: Scheduler = Scheduler::new();

// 安全地访问调度器的函数
#[allow(static_mut_refs)]
pub fn with_scheduler<F, R>(f: F) -> R
where
    F: FnOnce(&mut Scheduler) -> R,
{
    unsafe { f(&mut SCHEDULER) }
}

// 创建新任务

#[cfg(all(test, target_arch = "x86"))]
mod tests {
    use super::scheduler::*;

    #[test]
    fn test_create_task() {
        let mut scheduler = Scheduler::new();
        assert_eq!(scheduler.create_task("test_task", 1000, |_| {}), Ok(()));
        assert_eq!(scheduler.task_ready_list().len(), 1);
    }

    #[test]
    fn test_start() {
        let mut scheduler = Scheduler::new();
        scheduler.start();
        assert!(scheduler.idle_task().is_some());
        assert!(scheduler.current_task().is_none());
    }
    use core::fmt::{Debug, Error, Formatter};

    impl Debug for TCB {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(
                f,
                "TCB(name: {}, stack_ptr: {}, stack_size: {}, node_ptr: {:?}, unblock_time: {:?})",
                self.name, self.stack_ptr, self.stack_size, self.node_ptr, self.unblock_time
            )
        }
    }
    impl Eq for TCB {}
    impl PartialEq for TCB {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
                && self.stack_ptr == other.stack_ptr
                && self.stack_size == other.stack_size
                && self.node_ptr == other.node_ptr
                && self.unblock_time == other.unblock_time
        }
    }
    #[test]
    fn test_yield_task() {
        let mut scheduler = Scheduler::new();
        scheduler.create_task("task1", 1000, |_| {}).unwrap();
        scheduler.create_task("task2", 1000, |_| {}).unwrap();
        scheduler.start();

        let first_task = scheduler.current_task();
        //实际上是调用port_task_yield!()
        scheduler.task_switch_context();
        let second_task = scheduler.current_task();
        assert_ne!(first_task, second_task);
    }

    #[test]
    fn test_delay_task() {
        let mut scheduler = Scheduler::new();
        scheduler.create_task("task1", 1000, |_| {}).unwrap();
        scheduler.create_task("task2", 1000, |_| {}).unwrap();
        scheduler.start();

        // 现在就绪列表中应该有3个任务（2个创建的 + 1个空闲任务）
        assert_eq!(scheduler.task_ready_list().len(), 2);

        let original_task = scheduler.current_task();
        scheduler.delay_task(10);
        assert_ne!(scheduler.current_task(), original_task);
        assert_eq!(scheduler.task_delay_list().len(), 1);
        assert_eq!(scheduler.task_ready_list().len(), 1); // 2 = 3 - 1（被延迟的任务）
    }

    #[test]
    fn test_tick() {
        let mut scheduler = Scheduler::new();
        scheduler.create_task("task1", 1000, |_| {}).unwrap();
        scheduler.start();

        let original_ticks = scheduler.ticks_count();
        scheduler.tick();
        assert_eq!(scheduler.ticks_count(), original_ticks + 1);
    }

    #[test]
    fn test_unblock_tasks() {
        let mut scheduler = Scheduler::new();
        scheduler.create_task("task1", 1000, |_| {}).unwrap();
        scheduler.create_task("task2", 1000, |_| {}).unwrap();
        scheduler.start();

        // 现在就绪列表中应该有3个任务
        assert_eq!(scheduler.task_ready_list().len(), 2);

        scheduler.delay_task(5);
        assert_eq!(scheduler.task_delay_list().len(), 1);
        assert_eq!(scheduler.task_ready_list().len(), 1);

        for _ in 0..5 {
            scheduler.tick();
        }

        assert_eq!(scheduler.task_delay_list().len(), 0);
        assert_eq!(scheduler.task_ready_list().len(), 2); // 所有任务都应该回到就绪列表
    }
}
