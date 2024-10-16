use core::fmt;
extern crate alloc;
use alloc::string::String;
use core::fmt::Write;
pub static mut PRINT_FUNCTION: Option<fn(&str)> = None;
struct StringWriter(String);
impl Write for StringWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}
pub fn register_print_function(print_fn: fn(&str)) {
    unsafe {
        PRINT_FUNCTION = Some(print_fn);
    }
}

pub fn kernel_print(args: fmt::Arguments) {
    if let Some(print_fn) = unsafe { PRINT_FUNCTION } {
        let mut writer = StringWriter(String::new());
        writer.write_fmt(args).expect("Formatting failed");
        print_fn(&writer.0);
    }
}

#[macro_export]
macro_rules! kernel_println {
    () => ($crate::utils::print::kernel_print(format_args!("\n")));
    ($($arg:tt)*) => ($crate::utils::print::kernel_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kernel_print {
    ($($arg:tt)*) => ($crate::utils::print::kernel_print(format_args!($($arg)*)));
}
