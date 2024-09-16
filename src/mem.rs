use core::{mem, usize};

use alloc::alloc::*;

pub fn mem_alloc(size: usize) -> *mut u8 {
    // let layout = Layout::from_size_align(size, 1).unwrap();
    // let memory = unsafe { alloc(layout) };

    // if memory.is_null() {
    //     panic!();
    // }
    // memory
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

pub fn mem_free(ptr: *mut u8) {
    // if memory.is_null() {
    //     panic!();
    // }
    // unsafe {
    //     dealloc(memory, layout);
    // }

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
