// use core::ptr::NonNull;
// use link_node::LinkNode;
// use link_node::NodePtr;
// type ElementType = usize;
// type NodeType = NodePtr<ElementType>;
// type ListType = ();

pub mod ptr {
    use super::super::mem::mem::type_free;
    use super::super::mem::mem::type_malloc;
    use core::clone::Clone;
    use core::marker::Copy;
    use core::ops::{Deref, DerefMut};
    use core::ptr::NonNull;

    #[derive(PartialEq, Eq)]
    pub struct Ptr<T>(NonNull<T>);

    impl<T> Ptr<T> {
        pub fn new(data: T) -> Self {
            Ptr(type_malloc(data))
        }

        pub fn from_non_null(ptr: NonNull<T>) -> Self {
            Ptr(ptr)
        }
        pub fn free(&mut self) {
            type_free(self.0);
        }
    }

    impl<T> Clone for Ptr<T> {
        fn clone(&self) -> Self {
            Ptr(self.0)
        }
    }
    // impl<T> Drop for Ptr<T> {
    //     fn drop(&mut self) {
    //         type_free(self.0);
    //     }
    // }

    impl<T> Deref for Ptr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { self.0.as_ref() }
        }
    }

    impl<T> DerefMut for Ptr<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { self.0.as_mut() }
        }
    }
}

pub mod link_node {

    use super::ptr::Ptr;
    use core::clone::Clone;
    use core::marker::Copy;
    use core::ptr::NonNull;

    use core::option::Option;
    use core::option::Option::*;

    pub type ElementPtr<T> = Ptr<T>;
    pub type NodePtr<T> = Ptr<LinkNode<T>>;
    pub type ListPtr<T> = Ptr<LinkList<T>>;

    #[derive(Clone, PartialEq, Eq)]
    pub struct LinkNode<T> {
        pub data: Option<ElementPtr<T>>,
        pub next: Option<NodePtr<T>>,
        pub prev: Option<NodePtr<T>>,
        pub list_handle: Option<ListPtr<T>>,
    }

    // impl<T> Drop for LinkNode<T> {
    //     fn drop(&mut self) {
    //         // 释放数据
    //         if let Some(data) = self.data.take() {
    //             drop(data);
    //         }
    //         // 断开与其他节点的连接
    //         self.next = None;
    //         self.prev = None;
    //         self.list_handle = None;
    //     }
    // }

    impl<T> Drop for LinkList<T> {
        fn drop(&mut self) {
            while let Some(mut node) = self.head.take() {
                let next = node.next.clone();

                // 释放节点和数据
                node.data.as_mut().map(|data| data.free());

                node.free();

                self.head = next;
            }
            self.tail = None;
            self.len = 0;
        }
    }

    impl<T: Copy> LinkNode<T> {
        pub fn new(data: T) -> Self {
            Self {
                next: None,
                prev: None,
                data: Some(ElementPtr::new(data)),
                list_handle: None,
            }
        }
        fn next(&mut self) -> Option<NodePtr<T>> {
            self.next.clone()
        }

        fn prev(&mut self) -> Option<NodePtr<T>> {
            self.prev.clone()
        }

        fn set_next(&mut self, next: Option<NodePtr<T>>) {
            self.next = next;
        }

        fn set_prev(&mut self, prev: Option<NodePtr<T>>) {
            self.prev = prev;
        }

        fn set_element(&mut self, data: Option<ElementPtr<T>>) {
            self.data = data;
        }

        fn into_element(&self) -> Option<ElementPtr<T>> {
            self.data.clone()
        }

        fn get_list(&self) -> Option<ListPtr<T>> {
            self.list_handle.clone()
        }

        fn insert(&mut self, mut node: NodePtr<T>) {
            node.prev = self.prev.clone();

            node.next = self.next.clone();

            self.prev = Some(node);
        }

        fn del(&mut self) {
            if let Some(mut next) = self.next.clone() {
                next.prev = self.prev.clone();
            }
            if let Some(mut prev) = self.prev.clone() {
                prev.next = self.next.clone();
            }
            self.next = None;
            self.prev = None;
        }
    }
    #[derive(Clone, PartialEq, Eq)]
    pub struct LinkList<T> {
        tail: Option<NodePtr<T>>,
        head: Option<NodePtr<T>>,
        len: usize,
    }

    impl<T: Copy> LinkList<T> {
        pub fn new() -> ListPtr<T> {
            ListPtr::new(Self {
                head: None,
                tail: None,
                len: 0,
            })
        }
        pub fn push_back(&mut self, data: T) {
            let self_ptr = unsafe { NonNull::new_unchecked(self as *mut Self) };
            let new_node = NodePtr::new(LinkNode {
                data: Some(ElementPtr::new(data)),
                next: None,
                prev: self.tail.clone(),
                list_handle: Some(ListPtr::from_non_null(NonNull::from(self_ptr))),
            });

            match self.tail.take() {
                Some(mut old_tail) => {
                    old_tail.next = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
            self.len += 1;
        }
        pub fn push_front(&mut self, data: T) {
            let self_ptr = unsafe { NonNull::new_unchecked(self as *mut Self) };
            let new_node = NodePtr::new(LinkNode {
                data: Some(ElementPtr::new(data)),
                next: self.head.clone(),
                prev: None,
                list_handle: Some(ListPtr::from_non_null(self_ptr)),
            });

            match self.head.take() {
                Some(mut old_head) => {
                    old_head.prev = Some(new_node.clone());
                    self.head = Some(new_node);
                }
                None => {
                    self.head = Some(new_node.clone());
                    self.tail = Some(new_node);
                }
            }
            self.len += 1;
        }
        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|mut node| {
                self.len -= 1;

                if let Some(mut new_head) = node.next.clone() {
                    new_head.prev = None;
                    self.head = Some(new_head);
                } else {
                    self.tail = None;
                }
                let data = *node.data.clone().unwrap();
                node.data.take().unwrap().free();
                node.free();
                data
            })
        }
        pub fn pop_back(&mut self) -> Option<T> {
            self.tail.take().map(|mut node| {
                self.len -= 1;

                if let Some(mut new_tail) = node.prev.clone() {
                    new_tail.next = None;
                    self.tail = Some(new_tail);
                } else {
                    self.head = None;
                }
                let data = *node.data.clone().unwrap();
                node.data.take().unwrap().free();
                node.free();
                data
            })
        }

        pub fn front(&self) -> Option<T> {
            self.head
                .clone()
                .and_then(|node| node.data.clone().map(|elem_ptr| *elem_ptr))
        }

        pub fn back(&self) -> Option<T> {
            self.tail
                .clone()
                .and_then(|node| node.data.clone().map(|elem_ptr| *elem_ptr))
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }
}

#[cfg(test)]
mod tests {

    use super::link_node::*;

    impl<T: std::fmt::Debug + Copy> std::fmt::Debug for ElementPtr<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "NodePtr({:?})", self)
        }
    }

    #[test]
    fn test_node() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_link_list() {
        let mut list = LinkList::<usize>::new();
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        // 测试 push_front
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(10));
        assert_eq!(list.back(), Some(10));

        // 测试 push_back
        list.push_back(20);
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(10));
        assert_eq!(list.back(), Some(20));

        list.push_back(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(10));
        assert_eq!(list.back(), Some(30));

        // 测试 pop_front
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(20));
        assert_eq!(list.back(), Some(30));

        // 测试 pop_back
        assert_eq!(list.pop_back(), Some(30));
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(20));
        assert_eq!(list.back(), Some(20));

        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);

        // 额外测试：在空列表上操作
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }
}
