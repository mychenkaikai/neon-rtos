use alloc_cortex_m::CortexMHeap;
use cortex_m_rt::entry;
use cortex_m_rt::heap_start;

extern "C" {
    fn app_main() -> !;
}
// 全局分配器
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    unsafe {
        ALLOCATOR.init(heap_start() as usize, 1024 * 32);
    }
    unsafe {
        app_main();
    }
}
