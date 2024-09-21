use crate::mem::*;

use crate::task::*;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ptr::{addr_of, NonNull};

// pub static mut TASK_READY_LIST:List = List::new();
// pub static mut TASK_READY_LIST:List = List::new();
pub static mut TASK_READY_LIST: UnsafeCell<List> = UnsafeCell::new(List::new());
// pub static mut TASK_READY_LIST: UnsafeCell<Option<NonNull<List>>> = UnsafeCell::new(None);
// UnsafeCell<Option<NonNull<TCB>>>

pub static mut TASK_DELAY_LIST: UnsafeCell<List> = UnsafeCell::new(List::new());
// 获取 List 的可变引用
pub fn get_task_ready_list() -> &'static List {
    unsafe { &(*TASK_READY_LIST.get()) }
}

pub fn get_mut_task_ready_list() -> &'static mut List {
    unsafe { &mut (*TASK_READY_LIST.get_mut()) }
}

pub fn get_task_delay_list() -> &'static List {
    unsafe { &(*TASK_DELAY_LIST.get()) }
}

pub fn get_mut_task_delay_list() -> &'static mut List {
    unsafe { &mut (*TASK_DELAY_LIST.get_mut()) }
}

pub fn safely_modify_ready_list(f: impl FnOnce(&mut List)) {
    unsafe {
        let list = &mut *TASK_READY_LIST.get();
        f(list);
    }
}

pub fn safely_modify_list(mut list: NonNull<List>, f: impl FnOnce(&mut List)) {
    unsafe {
        let list = list.as_mut();
        f(list);
    }
}

pub fn safely_modify_delay_list(f: impl FnOnce(&mut List)) {
    unsafe {
        let list = &mut *TASK_DELAY_LIST.get();
        f(list);
    }
}

pub fn safely_delay_process_list<F>(mut process: F) -> Option<NonNull<TCB>>
where
    F: FnMut(usize, &mut TCB) -> bool, // 注意：需要使用 FnMut，因为闭包可能会修改自身状态
{
    unsafe {
        let list = get_mut_task_delay_list();
        let mut iter = list.into_iter(); // 使用迭代器遍历列表
        let mut index = 0;

        while let Some(mut item) = iter.next() {
            let list_item = item.as_mut(); // 获取可变引用
                                           // 执行处理操作
            if process(index, list_item) {
                return Some(list_item.self_handle); // 返回元素的句柄
            }
            index += 1;
        }
        None
    }
}

pub fn safely_ready_process_list<F>(mut process: F) -> Option<NonNull<TCB>>
where
    F: FnMut(usize, &mut TCB) -> bool, // 注意：需要使用 FnMut，因为闭包可能会修改自身状态
{
    unsafe {
        let list = get_mut_task_ready_list();
        let mut iter = list.into_iter(); // 使用迭代器遍历列表
        let mut index = 0;

        while let Some(mut item) = iter.next() {
            let list_item = item.as_mut(); // 获取可变引用
                                           // 执行处理操作
            if process(index, list_item) {
                return Some(list_item.self_handle); // 返回元素的句柄
            }
            index += 1;
        }
        None
    }
}

// // 获取 List 的可变引用
// pub fn get_mut_task_ready_list() -> Option<&'static mut List> {
//     unsafe { get_task_ready_list().map(|mut ptr| ptr.as_mut()) }
// }

pub struct List {
    pub len: usize,
    pub next: Option<NonNull<TCB>>,
    pub prev: Option<NonNull<TCB>>,
}
impl List {
    const fn new() -> Self {
        Self {
            prev: None,
            next: None,
            len: 0,
        }
    }

    pub fn get_first(&self) -> Option<NonNull<TCB>> {
        if self.len > 0 {
            return Some(self.next.unwrap());
        }
        None
    }

    // pub unsafe fn get_first_mut<'a>(&mut self) -> Option<NonNull<TCB>> {
    //     if self.len > 0 {
    //         return Some(self.next.unwrap());
    //     }
    //     None
    // }

    unsafe fn get_first_mut<'a>(&mut self) -> Option<&'a mut TCB> {
        if self.len > 0 {
            return Some(self.next.unwrap().as_mut());
        }
        None
    }

    pub fn set_next(&mut self, item: Option<NonNull<TCB>>) {
        self.next = item;
    }
    pub fn set_prev(&mut self, item: Option<NonNull<TCB>>) {
        self.prev = item;
    }

    fn join(&mut self, mut p_item: NonNull<TCB>) {
        let item = unsafe { p_item.as_mut() };

        if self.len == 0 {
            self.set_next(Some(p_item));
            self.set_prev(Some(p_item));
            item.set_next(None);
            item.set_prev(None);
            item.set_list(Some(NonNull::from(self)));
        } else {
            let mut p_old = self.next.unwrap();

            self.set_next(Some(p_item));

            item.set_next(Some(p_old));
            let old = unsafe { p_old.as_mut() };
            item.set_prev(None);

            old.set_prev(Some(p_item));
            item.set_list(Some(NonNull::from(self)));
        }
    }

    pub fn ins_to_first(&mut self, new_item: NonNull<TCB>) {
        self.join(new_item);

        self.len += 1;
    }

    pub fn del(&mut self, mut del_item: NonNull<TCB>) {
        if self.len == 0 {
            panic!()
        } else {
            unsafe {
                del_item.as_mut().del();
            }
            self.len -= 1;
        }
    }
    pub unsafe fn insert_tcb_into_list(tcb: NonNull<TCB>, list: &mut List) {
        // pub unsafe fn insert_tcb_into_list(tcb: NonNull<TCB>, mut list: NonNull<List>) {
        list.join(tcb);
        list.len += 1;
    }
    pub unsafe fn remove_tcb_from_list(tcb: NonNull<TCB>, mut list: NonNull<List>) {
        // pub unsafe fn remove_tcb_from_list(tcb: NonNull<TCB>, mut list: NonNull<List>) {
        list.as_mut().del(tcb);
        list.as_mut().len -= 1;
    }
}

pub struct TaskListIter<'a> {
    item: Option<NonNull<TCB>>,
    index: usize,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> IntoIterator for &'a List {
    type IntoIter = TaskListIter<'a>;
    type Item = NonNull<TCB>;
    fn into_iter(self) -> Self::IntoIter {
        TaskListIter {
            len: self.len,
            item: self.next,
            index: 0,
            _marker: PhantomData,
        }
    }
}

pub struct TaskListIterMut<'a> {
    item: Option<NonNull<TCB>>,
    // TASK_TABLE: &'a mut TaskTable,
    index: usize,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> IntoIterator for &'a mut List {
    type IntoIter = TaskListIterMut<'a>;
    type Item = NonNull<TCB>;

    fn into_iter(self) -> Self::IntoIter {
        TaskListIterMut {
            len: self.len,
            item: self.next,
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for TaskListIter<'a> {
    type Item = NonNull<TCB>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        if self.index >= self.len {
            return None;
        }

        match self.item {
            Some(ptcb) => {
                let ret = self.item;
                let tcb = unsafe { ptcb.as_ref() };
                self.item = tcb.next;
                return ret;
            }
            None => None,
        }
    }
}

impl<'a> Iterator for TaskListIterMut<'a> {
    type Item = NonNull<TCB>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        if self.index >= self.len {
            return None;
        }

        match self.item {
            Some(ptcb) => {
                let ret = self.item;
                let tcb = unsafe { ptcb.as_ref() };
                self.item = tcb.next;
                return ret;
            }
            None => None,
        }
    }
}
