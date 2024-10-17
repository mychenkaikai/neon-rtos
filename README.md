# Neon RTOS

Neon RTOS is a lightweight real-time operating system designed for ARM Cortex-M microcontrollers.

## Features

- Lightweight real-time operating system kernel
- Multi-task scheduling support
- Inter-task communication mechanisms
- Memory management
- Interrupt handling
- Support for ARM Cortex-M3 architecture

## Quick Start

### Requirements

- Rust toolchain (nightly)
- ARM GCC toolchain (for cross-compilation)
- QEMU (for emulation)

### Building and Running Examples

1. Navigate to the example directory:
   ```
   cd examples/basic
   ```

2. Build the example:
   ```
   cargo build --target thumbv7m-none-eabi
   ```

3. Run the example (using QEMU):
   ```
   cargo run --target thumbv7m-none-eabi
   ```

## Project Structure

- `src/`: Source code directory
  - `kernel/`: Kernel code
  - `arch/`: Architecture-specific code
  - `utils/`: Utility functions
- `examples/`: Example code
- `tests/`: Test code

## Configuration

Each example project has its own `Cargo.toml` and `.cargo/config.toml` files. You can adjust the target architecture and compilation options as needed.

## Testing

Run unit tests:
```
./test.sh
```


## Contributing

Pull Requests are welcome to improve this project. Before submitting, please ensure your code adheres to the project's coding standards and passes all tests.

## License

This project is dual-licensed under MIT or Apache-2.0. See the LICENSE file for details.