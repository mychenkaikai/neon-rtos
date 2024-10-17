# Neon RTOS

Neon RTOS 是一个为 ARM Cortex-M 微控制器设计的轻量级实时操作系统。

## 特性

- 轻量级实时操作系统内核
- 支持多任务调度
- 提供任务间通信机制
- 内存管理
- 中断处理
- 支持 ARM Cortex-M3 架构

## 快速开始

### 环境要求

- Rust 工具链 (nightly)
- ARM GCC 工具链 (用于交叉编译)
- QEMU (用于模拟运行)

### 编译和运行示例

1. 进入示例目录：
   ```
   cd examples/basic
   ```

2. 编译示例：
   ```
   cargo build --target thumbv7m-none-eabi
   ```

3. 运行示例（使用 QEMU）：
   ```
   cargo run --target thumbv7m-none-eabi
   ```

## 项目结构

- `src/`: 源代码目录
  - `kernel/`: 内核代码
  - `arch/`: 架构相关代码
  - `utils/`: 工具函数
- `examples/`: 示例代码
- `tests/`: 测试代码

## 配置

每个示例项目都有自己的 `Cargo.toml` 和 `.cargo/config.toml` 文件。您可以根据需要调整目标架构和编译选项。

## 测试

运行单元测试：
```
./test.sh
```


## 贡献

欢迎提交 Pull Requests 来改进这个项目。在提交之前，请确保您的代码符合项目的编码规范并通过所有测试。

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可。详情请参阅 LICENSE 文件。