pub mod mem {
    #[cfg(not(test))]
    use alloc::Box;
    #[cfg(test)]
    use std::boxed::Box;

    use core::ptr::NonNull;


    pub fn type_malloc<T>(data: T) -> NonNull<T> {
        let ptr = Box::new(data);
        NonNull::new(Box::leak(ptr)).unwrap()
    }

    pub fn type_free<T>(ptr: NonNull<T>) {
        unsafe {
            drop(Box::from_raw(ptr.as_ptr())); 
        }
    }
}
