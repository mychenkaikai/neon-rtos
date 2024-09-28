use core::{mem, usize,};
use core::alloc::Layout;
use alloc::alloc::*;
use core::ptr;

pub fn mem_alloc(size: usize) -> *mut u8 {
    let metadata_size = mem::size_of::<usize>();
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

pub fn mem_alloc_type<T>() -> *mut T {
    let layout = Layout::new::<T>();

    // 分配内存
    let ptr = unsafe { alloc(layout) as *mut T };

    // 确保分配成功（非空）
    if ptr.is_null() {
        panic!("Out of memory");
    }
    ptr
}

pub fn mem_free(ptr: *mut u8) {
    let metadata_size = mem::size_of::<usize>();
    let original_ptr = unsafe { ptr.sub(metadata_size) };

    // 使用 layout 恢复内存
    let layout = Layout::from_size_align(
        unsafe { original_ptr.cast::<usize>().read() + metadata_size },
        1,
    )
    .unwrap();
    unsafe { dealloc(original_ptr, layout) };
}

pub unsafe fn mem_free_type<T>(ptr: *mut T) {
    let layout: Layout = Layout::new::<T>();
    // 释放内存前，先调用析构函数（如果有）
    ptr::drop_in_place(ptr);
    // 释放内存
    dealloc(ptr as *mut u8, layout);
}
