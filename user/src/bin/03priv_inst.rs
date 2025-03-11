#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Try to execute privileged instructions in U mode");
    println!("Kernel should kill this application");
    
    // 执行特权指令错
    unsafe {
        asm!("sret");
    }
    0
}