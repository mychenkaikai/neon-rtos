use alloc::boxed::Box;

extern crate alloc;

use super::MemOperations;
use alloc::alloc::*;
use core::ptr::NonNull;
use core::mem::size_of;

pub struct ArchMem;

impl MemOperations for ArchMem {
    fn type_malloc<T>(data: T) -> NonNull<T> {
        let ptr = Box::new(data);
        NonNull::new(Box::leak(ptr)).unwrap()
    }

    fn type_free<T>(ptr: NonNull<T>) -> T {
        unsafe { *Box::from_raw(ptr.as_ptr()) }
    }

    fn mem_alloc(size: usize) -> *mut u8 {
        let metadata_size = size_of::<usize>();
        let total_size = size + metadata_size;

        // 创建 layout，包括元数据
        let layout = Layout::from_size_align(total_size, 1).unwrap();
        let memory = unsafe { alloc(layout) };

        // 将大小信息写入内存开头
        let metadata_ptr = memory as *mut usize;
        unsafe {
            metadata_ptr.write(size);
        }

        // 返回实际的数据部分
        unsafe { memory.add(metadata_size) }
    }

    fn mem_free(ptr: *mut u8) {
        let metadata_size = size_of::<usize>();
        let original_ptr = unsafe { ptr.sub(metadata_size) };

        // 使用 layout 恢复内存
        let layout = Layout::from_size_align(
            unsafe { original_ptr.cast::<usize>().read() + metadata_size },
            1,
        )
        .unwrap();
        unsafe { dealloc(original_ptr, layout) };
    }
}
