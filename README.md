# Neon RTOS

Neon RTOS 是一个为 ARM Cortex-M 或 RISC-V 微控制器设计的实时操作系统。

## 特性

- 轻量级实时操作系统内核
- 支持多任务调度
- 提供任务间通信机制
- 内存管理
- 中断处理
- 支持 ARM Cortex-M 架构

## 快速开始

### 环境要求

- Rust 工具链
- ARM GCC 工具链 (用于交叉编译)
- QEMU (用于模拟运行)

### 编译

使用以下命令编译项目:
cargo build --target thumbv7m-none-eabi --lib --example basic


### 运行示例

1. 使用 QEMU 运行示例:
qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv7m-none-eabi/debug/examples/basic


2. 或者使用 VS Code 的 Cortex-Debug 插件运行和调试 (参考 `.vscode/launch.json` 文件)

## 项目结构

- `src/`: 源代码目录
  - `kernel/`: 内核代码
  - `arch/`: 架构相关代码
  - `utils/`: 工具函数
- `examples/`: 示例代码
- `tests/`: 测试代码

## 配置

项目的主要配置在 `Cargo.toml` 和 `.cargo/config.toml` 文件中。您可以根据需要调整目标架构和编译选项。

## 测试

运行单元测试:
./test.sh


## 贡献

欢迎提交 Pull Requests 来改进这个项目。在提交之前，请确保您的代码符合项目的编码规范并通过所有测试。

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可。详情请参阅 LICENSE 文件。