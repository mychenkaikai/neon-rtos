use crate::task::LinkType::Tcb;
use crate::task::*;
use core::marker::PhantomData;
pub static mut TASK_READY_LIST: List = List::new();

pub static mut TASK_DELAY_LIST: List = List::new();

pub struct List {
    pub len: usize,
    pub link_node: LinkNode,
}
impl List {
    const fn new() -> Self {
        Self {
            link_node: LinkNode::new(),
            len: 0,
        }
    }

    pub unsafe fn get_first<'a>(&mut self) -> Option<&'a TCB> {
        if self.len > 0 {
            if let Tcb(id) = self.link_node.next {
                return Some(id.get());
            }
        }
        None
    }

    unsafe fn get_first_mut<'a>(&mut self) -> Option<&'a mut TCB> {
        if self.len > 0 {
            if let Tcb(id) = self.link_node.next {
                return Some(id.get_mut());
            }
        }
        None
    }

    pub unsafe fn ins_to_first(&mut self, new_item_id: TcbId) {
        if self.len > 0 {
            self.link_node.join(&mut new_item_id.get_mut().link_node);
        } else {
            self.link_node.join(&mut new_item_id.get_mut().link_node);
        }

        self.len += 1;
    }

    pub unsafe fn del(&mut self, del_item_id: TcbId) {
        if self.len == 0 {
            panic!()
        } else if self.len == 1 {
            self.link_node.next = self.link_node.id;
            self.link_node.prev = self.link_node.id;
            self.len = 0;
        } else {
            del_item_id.get_mut().link_node.del();
            self.len -= 1;
        }
    }
}

pub struct TaskListIter<'a> {
    node_id: LinkType,
    index: usize,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> IntoIterator for &'a List {
    type IntoIter = TaskListIter<'a>;
    type Item = &'a TCB;
    fn into_iter(self) -> Self::IntoIter {
        TaskListIter {
            len: self.len,
            node_id: self.link_node.id,
            index: 0,
            _marker: PhantomData,
        }
    }
}

pub struct TaskListIterMut<'a> {
    node_id: LinkType,
    // TASK_TABLE: &'a mut TaskTable,
    index: usize,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> IntoIterator for &'a mut List {
    type IntoIter = TaskListIterMut<'a>;
    type Item = &'a mut TCB;

    fn into_iter(self) -> Self::IntoIter {
        TaskListIterMut {
            len: self.len,
            node_id: self.link_node.next,
            index: 0,
            _marker: PhantomData,
        }
    }
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

        if let Tcb(a) = self.node_id {
            self.index += 1;
            self.node_id = a.get().link_node.next;
            return Some(a.get());
        };

        None
    }
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
        if let Tcb(a) = self.node_id {
            self.index += 1;
            self.node_id = a.get().link_node.next;
            return Some(a.get_mut());
        };

        None
    }
}
