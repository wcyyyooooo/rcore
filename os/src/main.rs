#![no_std]
#![no_main]
#![feature(panic_info_message)]
use core::arch::global_asm;
#[macro_use]
mod console;
mod lang_items;
mod sbi;
global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
fn print() {
    extern "C" {
        fn stext();
        fn etext();
        fn sdata();
        fn edata();
        fn srodata();
        fn erodata();
        fn sbss();
        fn ebss();
    }
    info!(
        "text range:[0x{:x} - 0x{:x}]",
        stext as usize, etext as usize
    );
    info!(
        "data range:[0x{:x} - 0x{:x}]",
        sdata as usize, edata as usize
    );
    info!(
        "rodata range:[0x{:x} - 0x{:x}]",
        srodata as usize, erodata as usize
    );
    info!("bss range:[0x{:x} - 0x{:x}]", sbss as usize, ebss as usize);
}
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    print();
    println!("Hello World");
    panic!("Shutdown machine!");
}
