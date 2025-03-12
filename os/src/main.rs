#![no_std]
#![no_main]

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod batch;
mod trap;
mod syscall;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        fn stext();
        fn etext();
        fn sdata();
        fn edata();
        fn srodata();
        fn erodata();
        fn sbss();
        fn ebss();
    }
    clear_bss();
    println!("Hello world!");
    error!("this is an error!");
    info!("this is an info! info: {}", "msg");
    debug!("this is a debug!");
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!("is able to show Chinese: 中文♥️");
    
    trap::init();
    batch::init();
    batch::run_next_app();
    // panic!("Shutdown machine!");
    
}

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0);
        }
    });
}