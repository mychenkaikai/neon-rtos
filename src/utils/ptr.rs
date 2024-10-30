use crate::arch::common::MemOperations;
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
