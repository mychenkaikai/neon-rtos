pub mod common;

#[cfg(feature = "cortex-m")]
pub mod port_cortex_m;

#[cfg(test)]
pub mod port_test;


// 定义一个公共的 port 模块
pub mod port {
    // 根据不同的特性重新导出相应的实现
    #[cfg(feature = "cortex-m")]
    pub use super::port_cortex_m::*;

    #[cfg(test)]
    pub use super::port_test::*;

    // #[cfg(test)]
    // pub use super::port_test::port_idle_task;

    // 如果没有启用任何特性，可以提供一个默认的空实现或者编译错误
    #[cfg(not(any(feature = "cortex-m", test)))]
    compile_error!("No port implementation selected. Please enable a feature.");
}