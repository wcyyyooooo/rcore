use crate::sbi::console_putchar;
use core::fmt::{self, Write};
struct Stdout;
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("{}{}{}\n", "\x1B[34m[INFO]", format_args!($fmt $(, $($arg)+)?), "\x1B[0m"));
    };
}
#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("{}{}{}\n", "\x1B[93m[WARN]", format_args!($fmt $(, $($arg)+)?), "\x1B[0m"));
    };
}
#[macro_export]
macro_rules! error{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("{}{}{}\n", "\x1B[31m[ERROR]", format_args!($fmt $(, $($arg)+)?), "\x1B[0m"));
    };
}
#[macro_export]
macro_rules! debug{
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("{}{}{}\n", "\x1B[32m[DEBUG]", format_args!($fmt $(, $($arg)+)?), "\x1B[0m"));
    };
}
