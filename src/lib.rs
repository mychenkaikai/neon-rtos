#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]



#[cfg(not(test))]
pub mod common;
#[cfg(not(test))]
pub mod interrupts;
// #[cfg(not(test))]
pub mod list;
#[cfg(not(test))]
pub mod mem;
#[cfg(not(test))]
pub mod port;
// #[cfg(not(test))]
pub mod task;
#[cfg(not(test))]
extern crate alloc;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        assert_eq!(2+2, 4);
    }

}
