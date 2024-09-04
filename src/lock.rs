#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock {
    locked: AtomicBool,
}

impl Spinlock {
    pub const fn new() -> Spinlock {
        Spinlock {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.locked.compare_and_swap(false, true, Ordering::Acquire) {}
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let spinlock = Spinlock::new();
    spinlock.lock();
    // Critical section
    spinlock.unlock();
}
