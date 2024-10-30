# Neon RTOS

Neon RTOS 是一个支持多架构的轻量级实时操作系统，目前支持 ARM Cortex-M 和 RISC-V 架构。

## 特性

- 轻量级实时操作系统内核
- 多任务调度支持
- 任务间通信机制（信号、互斥锁）
- 内存管理
- 中断处理
- 支持多架构（ARM Cortex-M3、RISC-V）
- 集成开发环境支持（VSCode 调试）

## 快速开始

### 环境要求

- Rust 工具链 (nightly)
- ARM GCC 工具链（用于 Cortex-M）或 RISC-V GCC（用于 RISC-V）
- VSCode + Cortex-Debug 插件（用于调试）
- QEMU（用于模拟运行）

### 编译和运行

#### 方式一：命令行编译

bash
进入示例目录
cd examples/basic
编译（Cortex-M）
cargo build --target thumbv7m-none-eabi
或编译（RISC-V）
cargo build --target riscv32imac-unknown-none-elf


#### 方式二：VSCode 调试
1. 打开 VSCode
2. 加载项目根目录
3. 按 F5 启动调试（已配置好 launch.json）

## 项目结构

- `src/`
  - `kernel/`: 内核代码
    - `scheduler/`: 调度器
    - `sync/`: 同步原语（互斥锁、信号）
    - `task/`: 任务管理
  - `arch/`: 架构相关代码
    - `port_cortex_m/`: ARM Cortex-M 实现
    - `port_riscv/`: RISC-V 实现
  - `user_api/`: 用户态接口
  - `utils/`: 工具函数
- `examples/`: 示例代码
  - `basic/`: 基础示例
  - `riscv/`: RISC-V 示例

## 调试

项目已配置好 VSCode 调试环境：
- 支持断点调试
- 支持变量查看
- 支持单步执行
- 支持 QEMU 模拟器

## 贡献

欢迎提交 Pull Requests 来改进这个项目。在提交之前，请确保：
1. 代码符合项目的编码规范
2. 通过所有测试
3. 更新相关文档

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可。详情请参阅 LICENSE 文件。