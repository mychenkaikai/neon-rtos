pub mod common;

#[cfg(all(feature = "cortex_m3", target_arch = "arm"))]
mod port_cortex_m;

#[cfg(all(feature = "riscv", target_arch = "riscv32"))]
mod port_riscv;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod port_test;

// 定义一个公共的 port 模块
pub mod port {
    // 根据不同的特性重新导出相应的实现
    #[cfg(all(feature = "cortex_m3", target_arch = "arm"))]
    pub use super::port_cortex_m::*;

    #[cfg(all(feature = "riscv", target_arch = "riscv32"))]
    pub use super::port_riscv::*;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub use super::port_test::*;

    // 如果没有启用任何特性，可以提供一个默认的空实现或者编译错误
    #[cfg(not(any(
        feature = "cortex_m3",
        feature = "riscv",
        test,
        target_arch = "x86",
        target_arch = "x86_64"
    )))]
    compile_error!("No port implementation selected. Please enable a feature.");
}
