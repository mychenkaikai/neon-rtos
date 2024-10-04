#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]



// #[cfg(not(test))]
pub mod common;
// #[cfg(not(test))]
pub mod interrupts;
// #[cfg(not(test))]
pub mod list;
// #[cfg(not(test))]
pub mod mem;
// #[cfg(not(test))]
pub mod port;
// #[cfg(not(test))]
pub mod task;

pub mod utils;

pub mod kernel;

pub mod arch_port;

#[cfg(not(test))]
extern crate alloc;

