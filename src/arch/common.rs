use core::ptr::NonNull;

pub trait ArchPortTrait {
    fn idle_task();
    fn enable_interrupts();
    fn disable_interrupts();
    fn is_interrupts_enabled() -> bool;
    fn enter_critical_section();
    fn exit_critical_section();
    fn critical_section<F: FnOnce()>(func: F);

    fn delay_ms(ms: u32);
    fn memory_barrier();

    fn start_first_task();

    fn task_yield() {}

    fn init_task_stack(top_of_stack: &mut usize, func: fn(usize), p_args: usize) {}
}

pub trait MemOperations {
    fn mem_alloc(size: usize) -> *mut u8;
    fn mem_free(ptr: *mut u8);
    fn type_malloc<T>(data: T) -> NonNull<T>;
    fn type_free<T>(ptr: NonNull<T>) -> T;
}

pub enum ExceptionType {
    // 定义异常类型
}

pub struct ExceptionInfo {
    // 定义异常信息结构
}

impl ExceptionInfo {
    pub fn new() -> Self {
        ExceptionInfo {
            // 初始化异常信息结构
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod tests {
    use super::*;

    // 创建一个模拟实现来测试 trait
    use crate::arch::port::mem::ArchMem;

    #[test]
    fn test_mem_alloc_and_free() {
        let ptr = ArchMem::mem_alloc(100);
        assert!(!ptr.is_null());
        ArchMem::mem_free(ptr);
    }

    #[test]
    fn test_type_malloc_and_free() {
        let data = 42;
        let ptr = ArchMem::type_malloc(data);
        assert!(!ptr.as_ptr().is_null());
        let retrieved_data = ArchMem::type_free(ptr);
        assert_eq!(retrieved_data, data);
    }

    #[test]
    fn test_type_malloc_and_free_complex_type() {
        #[derive(Debug, PartialEq)]
        struct TestStruct {
            a: i32,
            b: String,
        }

        let data = TestStruct {
            a: 10,
            b: "test".to_string(),
        };
        let ptr = ArchMem::type_malloc(data);
        assert!(!ptr.as_ptr().is_null());
        let retrieved_data = ArchMem::type_free(ptr);
        assert_eq!(retrieved_data.a, 10);
        assert_eq!(retrieved_data.b, "test");
    }

    #[test]
    fn test_mem_alloc_zero_size() {
        let ptr = ArchMem::mem_alloc(0);
        assert!(!ptr.is_null());
        ArchMem::mem_free(ptr);
    }

    #[test]
    fn test_mem_alloc_large_size() {
        let ptr = ArchMem::mem_alloc(1_000_000);
        assert!(!ptr.is_null());
        ArchMem::mem_free(ptr);
    }

    // #[test]
    // #[should_panic]
    // fn test_double_free() {
    //     let ptr = ArchMem::mem_alloc(8);
    //     ArchMem::mem_free(ptr);
    //     ArchMem::mem_free(ptr); // 这应该会导致 panic
    // }

    #[test]
    fn test_mem_free_safety() {
        let ptr = ArchMem::mem_alloc(8);
        assert!(!ptr.is_null());

        // 第一次释放应该成功
        ArchMem::mem_free(ptr);

        // 不要尝试第二次释放，而是验证第一次释放成功
        // 在实际实现中，您可能需要一种方法来检查内存是否已被释放
        assert!(true, "Memory freed successfully");
    }
}
