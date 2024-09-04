#![no_std]
#![no_main]
use cortex_m_rt::{ exception};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::ExceptionFrame;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use core::fmt::Write;
use core::ptr;
use cortex_m_semihosting::{debug, hprintln};
use core::fmt::{self};
#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }

    loop {}
}

#[entry]
fn main() -> ! {

    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    let a = 5;
    let mut b = 5;
    println!("123");
    Stdout.write_str("12344444");
    loop {
        // your code goes here
        // unsafe {
        //     // read an address outside of the RAM region; this causes a HardFault exception
        //     // ptr::read_volatile(0xFFFF_FFFF as *const u32);
            
        //     let c = a/b;
        //     b=b-1;
        // }
    }
}
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self,s:&str)->Result<(), core::fmt::Error>{
        // for c in s.chars(){
            
        // }
        hprintln!("{}",s);
        Ok(())
    }
}
pub fn print(args : fmt::Arguments)
{
    Stdout.write_fmt(args).unwrap();
}
#[macro_export]
macro_rules! println {
    ($fmt:literal $(,$($args:tt)+)?) => {
        $crate::print(format_args!($fmt $(,$($args:tt)+)?))
    };
}