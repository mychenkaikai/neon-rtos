// use core::ptr::NonNull;
// use link_node::LinkNode;
// use link_node::NodePtr;
// type ElementType = usize;
// type NodeType = NodePtr<ElementType>;
// type ListType = ();

pub mod ptr {
    use crate::arch::common::MemOperations;
    use crate::arch::port::*;
    use crate::arch::port::mem::ArchMem;
    // use crate::arch_port::port::;
    use core::clone::Clone;
    use core::marker::Copy;
    use core::ops::{Deref, DerefMut};
    use core::ptr::NonNull;

    #[derive(PartialEq, Eq)]
    pub struct Ptr<T>(pub NonNull<T>);

    impl<T> Ptr<T> {
        pub fn new(data: T) -> Self {
            Ptr(ArchMem::type_malloc(data))
        }

        pub fn from_non_null(ptr: NonNull<T>) -> Self {
            // Ptr(ptr.as_ptr())
            Ptr(ptr)
        }
        pub fn free_and_into_element(&mut self) -> T {
            ArchMem::type_free(self.0)
        }
    }
    impl<T> Copy for Ptr<T> {}

    impl<T> Clone for Ptr<T> {
        fn clone(&self) -> Self {
            Ptr(self.0)
        }
    }

    impl<T> Ptr<T> {
        pub fn as_ptr(&self) -> *mut T {
            self.0.as_ptr()
        }
    }

    // impl<T> Drop for Ptr<T> {
    //     fn drop(&mut self) {
    //         type_free(self.0);
    //     }
    // }

    impl<T> Deref for Ptr<T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &Self::Target {
            // unsafe { &*self.0 }
            unsafe { self.0.as_ref() }
        }
    }

    impl<T> DerefMut for Ptr<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            // unsafe { &mut *self.0 }
            unsafe { self.0.as_mut() }
        }
    }
}

pub mod link_node {

    use super::ptr::Ptr;
    use core::clone::Clone;
    use core::marker::Copy;
    use core::ops::Deref;
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

    // impl<T> Copy for LinkNode<T> {}

    impl<T> Drop for LinkList<T> {
        fn drop(&mut self) {
            while let Some(mut node) = self.head.take() {
                let next = node.next.take();

                // 释放节点和数据
                node.data.as_mut().map(|data| data.free_and_into_element());

                node.free_and_into_element();

                self.head = next;
            }
            self.tail = None;
            self.len = 0;
        }
    }

    impl<T> LinkNode<T> {
        pub fn new(data: T) -> Self {
            Self {
                next: None,
                prev: None,
                data: Some(ElementPtr::new(data)),
                list_handle: None,
            }
        }
        fn next(&mut self) -> Option<NodePtr<T>> {
            self.next
        }

        fn prev(&mut self) -> Option<NodePtr<T>> {
            self.prev
        }

        fn set_next(&mut self, next: Option<NodePtr<T>>) {
            self.next = next;
        }

        fn set_prev(&mut self, prev: Option<NodePtr<T>>) {
            self.prev = prev;
        }

        // fn set_element(&mut self, data: Option<ElementPtr<T>>) {
        //     self.data = data;
        // }

        // fn into_element(&self) -> Option<ElementPtr<T>> {
        //     self.data
        // }

        fn get_list(&self) -> Option<ListPtr<T>> {
            self.list_handle
        }

        fn insert(&mut self, mut node: NodePtr<T>) {
            node.prev = self.prev;

            node.next = self.next;

            self.prev = Some(node);
        }

        fn del(&mut self) {
            if let Some(mut next) = self.next {
                next.prev = self.prev;
            }
            if let Some(mut prev) = self.prev {
                prev.next = self.next;
            }
            self.next = None;
            self.prev = None;
        }
    }
    #[derive(Clone, PartialEq, Eq)]
    pub struct LinkList<T> {
        pub tail: Option<NodePtr<T>>,
        pub head: Option<NodePtr<T>>,
        len: usize,
    }

    // impl<T: Copy> Copy for LinkList<T> where T: Copy {}

    impl<T> LinkList<T> {
        pub const fn new() -> LinkList<T> {
            Self {
                head: None,
                tail: None,
                len: 0,
            }
        }
        #[inline]
        pub fn push_back(&mut self, data: T) -> NodePtr<T> {
            let self_ptr = unsafe { NonNull::new_unchecked(self as *mut Self) };
            let new_node = NodePtr::new(LinkNode {
                data: Some(ElementPtr::new(data)),
                next: None,
                prev: self.tail,
                list_handle: Some(ListPtr::from_non_null(self_ptr)),
            });

            match self.tail.take() {
                Some(mut old_tail) => {
                    old_tail.next = Some(new_node);
                    self.tail = Some(new_node);
                }
                None => {
                    self.head = Some(new_node);
                    self.tail = Some(new_node);
                }
            }
            self.len += 1;
            new_node
        }
        #[inline]
        pub fn push_front(&mut self, data: T) -> NodePtr<T> {
            let self_ptr = unsafe { NonNull::new_unchecked(self as *mut Self) };
            let new_node = NodePtr::new(LinkNode {
                data: Some(ElementPtr::new(data)),
                next: self.head,
                prev: None,
                list_handle: Some(ListPtr::from_non_null(self_ptr)),
            });

            match self.head.take() {
                Some(mut old_head) => {
                    old_head.prev = Some(new_node);
                    self.head = Some(new_node);
                }
                None => {
                    self.head = Some(new_node);
                    self.tail = Some(new_node);
                }
            }
            self.len += 1;
            new_node
        }
        #[inline]
        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|mut node| {
                self.len -= 1;

                if let Some(mut new_head) = node.next {
                    new_head.prev = None;
                    self.head = Some(new_head);
                } else {
                    self.tail = None;
                }
                let data = node.data.unwrap().free_and_into_element();
                node.free_and_into_element();
                data
            })
        }
        #[inline]
        pub fn pop_back(&mut self) -> Option<T> {
            self.tail.take().map(|mut node| {
                self.len -= 1;

                if let Some(mut new_tail) = node.prev {
                    new_tail.next = None;
                    self.tail = Some(new_tail);
                } else {
                    self.head = None;
                }
                let data = node.data.unwrap().free_and_into_element();
                node.free_and_into_element();
                data
            })
        }

        pub fn front(&self) -> Option<&T> {
            self.head.as_deref().and_then(|node| node.data.as_deref())
        }

        pub fn back(&self) -> Option<&T> {
            self.tail.as_deref().and_then(|node| node.data.as_deref())
        }

        pub fn detach(&mut self, mut node: NodePtr<T>) -> NodePtr<T> {
            if let Some(mut prev) = node.prev {
                prev.next = node.next;
            } else {
                self.head = node.next;
            }
    
            if let Some(mut next) = node.next {
                next.prev = node.prev;
            } else {
                self.tail = node.prev;
            }
    
            self.len -= 1;
    
            // 重置节点的前后指针
            node.prev = None;
            node.next = None;
    
            node
        }
    
        // 将已存在的节点添加到列表末尾
        pub fn attach_back(&mut self, mut node: NodePtr<T>) {
            node.prev = self.tail;
            node.next = None;
    
            if let Some(mut tail) = self.tail {
                tail.next = Some(node);
            } else {
                self.head = Some(node);
            }
    
            self.tail = Some(node);
            self.len += 1;
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            if self.len == 0 {
                return true;
            } else {
                return false;
            }
        }

        pub fn iter(&self) -> LinkListIter<T> {
            LinkListIter {
                next: self.head.as_ref(),
            }
        }
    }

    pub struct LinkListIter<'a, T> {
        next: Option<&'a NodePtr<T>>,
    }

    impl<'a, T> IntoIterator for &'a LinkList<T> {
        type Item = &'a T;
        type IntoIter = LinkListIter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            LinkListIter {
                next: self.head.as_ref(),
            }
        }
    }

    impl<'a, T> Iterator for LinkListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.and_then(|node| {
                self.next = node.next.as_ref();
                node.data.as_deref()
            })
        }
    }

    pub trait Linkable: Sized {
        fn get_node_ptr(&self) -> Option<NodePtr<Self>>;
        fn set_node_ptr(&mut self, ptr: Option<NodePtr<Self>>);
    }
}

pub mod marco {

    use super::link_node::*;

    #[macro_export]
    macro_rules! linkable {
        ($name:ident) => {
            impl Linkable for $name {
                fn get_node_ptr(&self) -> Option<NodePtr<Self>> {
                    self.node_ptr
                }

                fn set_node_ptr(&mut self, ptr: Option<NodePtr<Self>>) {
                    self.node_ptr = ptr;
                }
            }
        };
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod tests {

    use super::*;
    use core::fmt::Debug;
    use core::sync::atomic::{AtomicUsize, Ordering};

    static INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, PartialEq)]
    struct DropCounter(i32);

    impl DropCounter {
        fn new(value: i32) -> Self {
            INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);
            DropCounter(value)
        }
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            INSTANCE_COUNT.fetch_sub(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_memory_leak() {
        {
            let mut list = LinkList::<DropCounter>::new();
            for i in 0..100 {
                list.push_back(DropCounter::new(i));
            }
            assert_eq!(INSTANCE_COUNT.load(Ordering::SeqCst), 100);

            // 移除一些元素
            for _ in 0..50 {
                list.pop_front();
            }
            assert_eq!(INSTANCE_COUNT.load(Ordering::SeqCst), 50);
        }
        // 列表超出作用域，应该释放所有剩余元素
        assert_eq!(INSTANCE_COUNT.load(Ordering::SeqCst), 0);
    }

    use super::link_node::*;

    impl<T: core::fmt::Debug> core::fmt::Debug for ElementPtr<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "ElementPtr({:p})", self.0.as_ptr())
        }
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
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&10));

        // 测试 push_back
        list.push_back(20);
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&20));

        list.push_back(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&30));

        // 测试 pop_front
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&20));
        assert_eq!(list.back(), Some(&30));

        // 测试 pop_back
        assert_eq!(list.pop_back(), Some(30));
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(&20));
        assert_eq!(list.back(), Some(&20));

        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);

        // 额外测试：在空列表上操作
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        #[derive(PartialEq, Eq, Debug)]
        struct T1 {
            a: usize,
            b: usize,
        }

        let mut list = LinkList::<T1>::new();
        list.push_back(T1 { a: 1, b: 2 });
        list.push_back(T1 { a: 3, b: 4 });
        list.push_back(T1 { a: 5, b: 6 });
        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&T1 { a: 1, b: 2 }));
        assert_eq!(list.back(), Some(&T1 { a: 5, b: 6 }));
    }
    #[test]
    fn test_link_list_comprehensive() {
        let mut list = LinkList::<i32>::new();

        // 测试空列表
        assert!(list.is_empty());

        // 测试插入和删除
        list.push_front(1);
        list.push_back(2);
        list.push_front(0);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.len(), 1);

        // 测试清空列表
        list.push_back(3);
        list.push_back(4);
        while list.pop_front().is_some() {}
        assert!(list.is_empty());

        // 测试大量数据
        for i in 0..1000 {
            list.push_back(i);
        }
        assert_eq!(list.len(), 1000);
        for i in 0..1000 {
            assert_eq!(list.pop_front(), Some(i));
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_link_list_iterator() {
        let mut list = LinkList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut sum = 0;
        for &item in &list {
            sum += item;
        }
        assert_eq!(sum, 6);

        // 或者使用 iter() 方法
        let sum: i32 = list.iter().sum();
        assert_eq!(sum, 6);
    }

    use super::link_node::ListPtr;
    use crate::linkable;
    use core::ptr::NonNull;

    #[test]
    fn test_linkable_macro() {
        // 定义一个测试用的结构体
        #[derive(PartialEq, Eq, Debug)]
        struct TestStruct {
            value: i32,
            node_ptr: Option<NodePtr<Self>>,
        }

        impl<T: std::fmt::Debug> std::fmt::Debug for LinkNode<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("LinkNode")
                    .field("data", &self.data)
                    .field(
                        "next",
                        &format_args!("{:p}", self.next.as_ref().map_or(std::ptr::null(), |n| n)),
                    )
                    .field(
                        "prev",
                        &format_args!("{:p}", self.prev.as_ref().map_or(std::ptr::null(), |n| n)),
                    )
                    .field(
                        "list_handle",
                        &format_args!(
                            "{:p}",
                            self.list_handle
                                .as_ref()
                                .map_or(std::ptr::null(), |l| l.0.as_ptr())
                        ),
                    )
                    .finish()
            }
        }
        impl<T> std::fmt::Debug for LinkList<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "ListPtr({:p})", self)
            }
        }

        // 使用宏为 TestStruct 实现 Linkable
        linkable!(TestStruct);

        // 创建一个 LinkList<TestStruct>
        let mut list = LinkList::<TestStruct>::new();

        // 添加一些 TestStruct 实例到列表中
        let node = list.push_back(TestStruct {
            value: 1,
            node_ptr: None,
        });

        if let Some(mut element) = node.data {
            element.set_node_ptr(Some(node));
        }
        let node = list.push_back(TestStruct {
            value: 2,
            node_ptr: None,
        });
        if let Some(mut element) = node.data {
            element.set_node_ptr(Some(node));
        }
        let node = list.push_back(TestStruct {
            value: 3,
            node_ptr: None,
        });
        if let Some(mut element) = node.data {
            element.set_node_ptr(Some(node));
        }

        // 验证列表长度
        assert_eq!(list.len(), 3);

        // 检查每个元素的 node_ptr
        for (index, item) in list.iter().enumerate() {
            println!(
                "Item {}: value = {}, node_ptr = {:?}",
                index,
                item.value,
                item.get_node_ptr()
            );
            assert!(
                item.get_node_ptr().is_some(),
                "Item {} should have a node_ptr",
                index
            );
        }

        if let Some(first) = list.front() {
            let node_ptr = first.get_node_ptr();
            assert!(node_ptr.is_some(), "First element should have a node_ptr");

            if let Some(node) = node_ptr {
                if let Some(next_node) = node.next {
                    assert_eq!(next_node.data.as_ref().unwrap().value, 2);
                } else {
                    panic!("Next node should exist");
                }
            }
        } else {
            panic!("List should not be empty");
        }

        if let Some(first) = list.back() {
            let node_ptr = first.get_node_ptr();
            assert!(node_ptr.is_some(), "First element should have a node_ptr");

            if let Some(node) = node_ptr {
                if let Some(_) = node.next {
                    panic!("Next node should  not exist");
                }
            }
        } else {
            panic!("List should not be empty");
        }

        // 测试移除元素
        let removed = list.pop_front();
        assert_eq!(removed.map(|ts| ts.value), Some(1));
        assert_eq!(list.len(), 2);

        // 再次检查剩余元素
        let values: Vec<i32> = list.iter().map(|ts| ts.value).collect();
        assert_eq!(values, vec![2, 3]);

        // 清空列表
        while list.pop_front().is_some() {}
        assert_eq!(list.len(), 0);
    }
}
