# Neon RTOS

Neon RTOS is a lightweight real-time operating system that supports multiple architectures, currently including ARM Cortex-M and RISC-V.

## Features

- Lightweight real-time operating system kernel
- Multi-task scheduling
- Inter-task communication (signals, mutexes)
- Memory management
- Interrupt handling
- Multi-architecture support (ARM Cortex-M3, RISC-V)
- Integrated development environment support (VSCode debugging)

## Quick Start

### Requirements

- Rust toolchain (nightly)
- ARM GCC toolchain (for Cortex-M) or RISC-V GCC (for RISC-V)
- VSCode + Cortex-Debug extension (for debugging)
- QEMU (for emulation)

### Building and Running

#### Option 1: Command Line Build
```bash
# Navigate to example directory
cd examples/basic

# Build for Cortex-M
cargo build --target thumbv7m-none-eabi

# Or build for RISC-V
cargo build --target riscv32imac-unknown-none-elf
```

#### Option 2: VSCode Debugging
1. Open VSCode
2. Load the project root directory
3. Press F5 to start debugging (launch.json is pre-configured)

## Project Structure

- `src/`
  - `kernel/`: Kernel code
    - `scheduler/`: Task scheduler
    - `sync/`: Synchronization primitives (mutexes, signals)
    - `task/`: Task management
  - `arch/`: Architecture-specific code
    - `port_cortex_m/`: ARM Cortex-M implementation
    - `port_riscv/`: RISC-V implementation
  - `user_api/`: User-space interface
  - `utils/`: Utility functions
- `examples/`: Example code
  - `basic/`: Basic examples
  - `riscv/`: RISC-V examples

## Debugging

The project is configured for VSCode debugging environment:
- Breakpoint debugging
- Variable inspection
- Step-by-step execution
- QEMU emulator support

## Contributing

Pull Requests are welcome to improve this project. Before submitting, please ensure:
1. Code adheres to the project's coding standards
2. All tests pass
3. Related documentation is updated

## License

This project is dual-licensed under MIT or Apache-2.0. See the LICENSE file for details.