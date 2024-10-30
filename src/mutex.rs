use crate::scheduler::TCB;
use crate::utils::ptr::Ptr;
extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

use spin::lazy::Lazy;

static NEXT_MUTEX_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Mutex {
    inner: Lazy<InnerMutex>,
}

struct InnerMutex {
    id: usize,
}

impl Mutex {
    pub const fn new() -> Self {
        Mutex {
            inner: Lazy::new(|| InnerMutex {
                id: NEXT_MUTEX_ID.fetch_add(1, Ordering::SeqCst),
            }),
        }
    }

    pub fn lock(&self) {
        crate::syscall::task_mutex_lock(self.inner.id);
    }

    pub fn unlock(&self) {
        crate::syscall::task_mutex_unlock(self.inner.id);
    }
}

pub struct MutexManager {
    mutex_queues: Vec<Vec<Ptr<TCB>>>,    // 每个互斥锁的等待队列
    mutex_owners: Vec<Option<Ptr<TCB>>>, // 每个互斥锁的当前持有者
}

impl MutexManager {
    pub const fn new() -> Self {
        MutexManager {
            mutex_queues: Vec::new(),
            mutex_owners: Vec::new(),
        }
    }

    pub fn lock(&mut self, mutex_id: usize, task: Ptr<TCB>) -> bool {
        const MAX_MUTEX_COUNT: usize = 32;
        // 确保有足够的空间
        if mutex_id >= MAX_MUTEX_COUNT {
            return false;
        }

        // 确保有足够的空间，但有上限
        while self.mutex_owners.len() <= mutex_id && self.mutex_owners.len() < MAX_MUTEX_COUNT {
            self.mutex_owners.push(None);
            self.mutex_queues.push(Vec::with_capacity(8)); // 预分配合理大小
        }

        // 如果锁已被占用
        if let Some(owner) = self.mutex_owners[mutex_id] {
            // 如果当前任务已经持有锁，返回true
            if owner.as_ptr() == task.as_ptr() {
                return true;
            }
            // 如果其他任务持有锁，将当前任务加入等待队列
            self.mutex_queues[mutex_id].push(task);
            return false;
        }

        // 锁未被占用，将当前任务设为所有者
        self.mutex_owners[mutex_id] = Some(task);
        true
    }

    pub fn unlock(&mut self, mutex_id: usize) -> Option<Ptr<TCB>> {
        if mutex_id >= self.mutex_owners.len() {
            return None;
        }

        // 清除当前所有者
        self.mutex_owners[mutex_id] = None;

        // 从等待队列中取出下一个任务并设为新的所有者
        if let Some(next_task) = self.mutex_queues[mutex_id].pop() {
            self.mutex_owners[mutex_id] = Some(next_task);
            Some(next_task)
        } else {
            None
        }
    }
}
