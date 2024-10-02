pub mod mem {
    #[cfg(not(test))]
    use alloc::Box;
    #[cfg(test)]
    use std::boxed::Box;

    use core::ptr::NonNull;

    pub fn type_malloc<T>(data: T) -> *mut T {
        let ptr = Box::new(data);
        // NonNull::new(Box::leak(ptr)).unwrap()
        Box::into_raw(ptr)
    }

    pub fn type_free<T>(ptr: *mut T) -> T {
        unsafe { *(Box::from_raw(ptr)) }
    }
}
