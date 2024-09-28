#[cfg(not(test))]
pub mod task {
    use alloc::boxed::Box;
    use cortex_m::asm::nop;

    use core::ptr::NonNull;
    use core::usize;

    use core::result::Result;

    use crate::list::list::*;
    use crate::mem::mem::mem_alloc;
    use crate::task_yield;

    use core::cell::UnsafeCell;
    use core::mem;
    use cortex_m_semihosting::hprintln;

    static mut NEXT_DELAY_TASK_UNBLOCK_TIME: UnsafeCell<Option<usize>> = UnsafeCell::new(None);
    // static mut TASK_TABLE: TaskTable = TaskTable::new();

    #[no_mangle]
    pub static mut CURRENT_TASK: UnsafeCell<Option<NonNull<TCB>>> = UnsafeCell::new(None);

    impl<'a> Into<&'a TCB> for NonNull<TCB> {
        fn into(self) -> &'a TCB {
            unsafe { self.as_ref() }
        }
    }

    pub fn get_mut_current_task() -> &'static mut Option<NonNull<TCB>> {
        unsafe { &mut (*CURRENT_TASK.get_mut()) }
    }

    pub fn set_current_task(tcb: NonNull<TCB>) {
        let a = get_mut_current_task();
        *a = Some(tcb);
    }

    pub fn get_unblock_time<'a>() -> &'a mut Option<usize> {
        unsafe { &mut (*NEXT_DELAY_TASK_UNBLOCK_TIME.get_mut()) }
    }

    pub unsafe fn get_list_of_tcb(tcb: NonNull<TCB>) -> Option<NonNull<List>> {
        tcb.as_ref().list_handle
    }

    pub fn safely_modify_current(f: impl FnOnce(&mut Option<NonNull<TCB>>)) {
        unsafe {
            let list = &mut *CURRENT_TASK.get();
            f(list);
        }
    }

    pub fn safely_modify_idle(f: impl FnOnce(&mut Option<NonNull<TCB>>)) {
        unsafe {
            let list = &mut *IDLE_TASK_TCB.get();
            f(list);
        }
    }

    fn get_idle_task() -> &'static Option<NonNull<TCB>> {
        unsafe { &(*IDLE_TASK_TCB.get()) }
    }

    pub fn safely_modify_unblock_time(f: impl FnOnce(&mut Option<usize>)) {
        unsafe {
            let list = &mut *NEXT_DELAY_TASK_UNBLOCK_TIME.get();
            f(list);
        }
    }

    pub fn safely_modify_tcb(mut tcb: NonNull<TCB>, f: impl FnOnce(&mut TCB)) {
        unsafe {
            let tcb = tcb.as_mut();
            f(tcb);
        }
    }

    #[derive(PartialEq)]
    #[repr(C)]
    pub struct TCB {
        pub top_of_stack: usize,
        pub stack_addr: usize,
        pub name: &'static str,
        pub stack_size: usize,
        pub next: Option<NonNull<TCB>>,
        pub prev: Option<NonNull<TCB>>,
        pub item_value: Option<usize>,
        pub self_handle: NonNull<TCB>,
        pub list_handle: Option<NonNull<List>>,
        // pub link_node: LinkNode,
    }
    impl TCB {
        pub fn new() -> NonNull<TCB> {
            let value = Self {
                top_of_stack: 0,
                stack_addr: 0,
                name: "none",
                stack_size: 0,
                prev: None,
                next: None,
                item_value: None,
                self_handle: NonNull::dangling(),
                list_handle: None,
            };
            let mut ptr = NonNull::from(Box::leak(Box::new(value)));
            unsafe {
                ptr.as_mut().self_handle = ptr;
            }
            ptr
        }

        pub fn set_next(&mut self, item: Option<NonNull<TCB>>) {
            self.next = item;
        }
        pub fn set_prev(&mut self, item: Option<NonNull<TCB>>) {
            self.prev = item;
        }

        pub unsafe fn join(&mut self, mut item: NonNull<TCB>) {
            // self.set_next(Some(item));
            // self.set_prev(Some(item));

            if self.next != None {
                let mut old_next = self.next.unwrap();

                self.set_next(Some(item));

                item.as_mut().set_next(Some(old_next));
                item.as_mut().set_prev(Some(self.self_handle));

                old_next.as_mut().set_prev(Some(item));
            } else {
                self.set_next(Some(item));

                item.as_mut().set_next(None);
                item.as_mut().set_prev(Some(self.self_handle));

                self.list_handle.unwrap().as_mut().set_prev(Some(item));
            }
            //todo
            item.as_mut().list_handle = self.list_handle;
        }

        pub fn set_list(&mut self, item: Option<NonNull<List>>) {
            self.list_handle = item;
        }

        pub unsafe fn del(&mut self) {
            let list: &mut List = self.list_handle.unwrap().as_mut();
            if self.prev.is_none() {
                match self.next {
                    Some(mut next_item) => {
                        list.set_next(Some(next_item));
                        next_item.as_mut().set_prev(None);
                        self.set_next(None);
                    }
                    None => list.set_next(None),
                }
            } else {
                let prev_item = self.prev;

                match self.next {
                    Some(mut next_item) => {
                        prev_item.unwrap().as_mut().set_next(Some(next_item));
                        next_item.as_mut().set_prev(prev_item);
                        self.set_next(None);
                    }
                    None => {
                        prev_item.unwrap().as_mut().set_next(None);
                        list.set_prev(prev_item);
                    }
                }
                self.set_prev(None);
            }
            self.list_handle = None;
        }
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
    }

    pub fn create_task(
        func: fn(usize),
        task_name: &'static str,
        size: usize,
        arg: usize,
    ) -> Result<(), &'static str> {
        // disable_interrupts();

        let tcb = TCB::new();

        let memory = mem_alloc(size);
        create_static_task(func, task_name, arg, memory as usize, size, tcb).unwrap();

        safely_modify_ready_list(|list| list.ins_to_first(tcb));

        // enable_interrupts();
        Ok(())
        // 使用 `memory` 进行读写操作...

        // 使用完后释放内存
    }

    pub fn create_idle_task() -> Result<(), &'static str> {
        let size = 500;
        let func = idle_task;
        let task_name: &'static str = "idle";
        let arg = 0;
        // disable_interrupts();

        let tcb = TCB::new();

        safely_modify_idle(|idle| *idle = Some(tcb));

        let memory = mem_alloc(size);
        create_static_task(func, task_name, arg, memory as usize, size, tcb).unwrap();

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
        tcb: NonNull<TCB>,
    ) -> Result<(), &'static str> {
        // disable_interrupts();

        let mut top_of_stack = stack_addr as usize + (size - 1);
        top_of_stack = top_of_stack & (!(0x0007));

        safely_modify_tcb(tcb, |tcb| {
            tcb.top_of_stack = top_of_stack;
            tcb.stack_addr = stack_addr as usize;
            tcb.name = task_name;
            tcb.stack_size = size;
            init_task_stack(&mut tcb.top_of_stack, func, arg);
            hprintln!(
                "task: start{:x} top{:x} top{:x} ",
                stack_addr as usize,
                tcb.top_of_stack,
                stack_addr as usize + (size - 1)
            )
            .unwrap();
        });

        // enable_interrupts();
        Ok(())
    }

    fn idle_task(_arg: usize) {
        loop {
            cortex_m::asm::wfi();
        }
    }

    static mut IDLE_TASK_TCB: UnsafeCell<Option<NonNull<TCB>>> = UnsafeCell::new(None);
    pub fn scheduler() {
        create_idle_task().unwrap();

        safely_modify_current(|curr_task| {
            if let None = curr_task {
                *curr_task = safely_ready_process_list(|index, tcb| true);
            }

            safely_modify_tcb(curr_task.unwrap(), |item| {
                hprintln!("now task is name is {}", item.name).unwrap();
            });
        });

        safely_modify_ready_list(|rlist| {
            hprintln!("total task is {}", rlist.len).unwrap();
        });
    }
    #[no_mangle]
    pub fn task_switch_context() {
        safely_modify_current(|curr| {
            if let Some(_curr) = curr {
                if *_curr == get_idle_task().unwrap() {
                    safely_modify_ready_list(|list| {
                        if list.len > 0 {
                            set_current_task(list.get_first().unwrap());
                        }
                    });
                } else {
                    // let cur = _current_task.as_ref();
                    // safely_modify_tcb(*_curr, |tcb| match tcb.next {
                    //     Some(next_item) => set_current_task(next_item),
                    //     None => {
                    //         safely_modify_ready_list(|list| {
                    //             if list.len > 0 {
                    //                 set_current_task(list.get_first().unwrap());
                    //             } else {
                    //                 set_current_task(get_idle_task().unwrap());
                    //             }
                    //         });
                    //     }
                    // });

                    safely_modify_ready_list(|list| {
                        if list.len > 0 {
                            set_current_task(list.get_first().unwrap());
                        } else {
                            set_current_task(get_idle_task().unwrap());
                        }
                    });
                }
            } else {
                nop();
            }
        })

        // hprintln!("switch {:x}", (*CURRENT_TASK.unwrap()).top_of_stack).unwrap();
    }

    // #[allow(unused)]
    // pub fn task_delay(ms_to_delay: usize) {
    //     if let Some(mut task) = get_mut_current_task() {
    //         // 防止溢出
    //         if ms_to_delay > usize::MAX / 1000 {
    //             panic!("Delay time is too large and may cause overflow");
    //         }
    //         // todo
    //         let tick_to_delay = ms_to_delay / 1000 * (crate::SYST_FREQ as usize);

    //         let current_tick = unsafe { TICKS_COUNT as usize };
    //         let expect_tick = current_tick.checked_add(tick_to_delay).unwrap_or_else(|| {
    //             panic!("Adding ticks to current tick caused an overflow");
    //         });

    //         let mut task_ref = unsafe { task.as_mut() };

    //         task_ref.item_value = Some(expect_tick);

    //         if expect_tick < current_tick {
    //             // 处理不合理的情况
    //             hprintln!("Warning: Delay time is negative, ignoring delay.");
    //             task_yield!(); // 直接调度任务
    //         } else {
    //             safely_modify_unblock_time(|unblock_time| {
    //                 *unblock_time = Some(expect_tick);
    //             });
    //             // disable_interrupts();
    //             safely_modify_ready_list(|rlist| {
    //                 rlist.del(task);
    //             });

    //             safely_modify_delay_list(|dlist| {
    //                 dlist.ins_to_first(task);
    //             });
    //             // enable_interrupts();
    //         }
    //     }

    //     task_yield!(); // 修正拼写错误
    // }
    static mut TICKS_COUNT: usize = 0;
    pub fn systick_task_inc() {
        unsafe {
            TICKS_COUNT += 1;
        }
        let mut should_switch = false;
        {
            if let Some(expect_time) = get_unblock_time() {
                let current_time = unsafe { TICKS_COUNT as usize };
                if current_time >= *expect_time {
                    loop {
                        if get_task_delay_list().len == 0 {
                            *(get_unblock_time()) = None;
                            // NEXT_DELAY_TASK_UNBLOCK_TIME = None;
                            break;
                        } else {
                            // disable_interrupts();
                            if let Some(item) = safely_delay_process_list(|index, tcb| true) {
                                safely_modify_delay_list(|dlist| {
                                    dlist.del(item);
                                });

                                safely_modify_ready_list(|rlist| {
                                    rlist.ins_to_first(item);
                                });
                            }
                            // enable_interrupts();
                        }
                    }
                }
            }

            if get_mut_task_ready_list().len > 0 {
                should_switch = true;
            }
        }
        if should_switch {
            task_yield!();
        }
    }
}
