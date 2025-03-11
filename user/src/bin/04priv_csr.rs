#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use riscv::register::sstatus::{self, SPP};

#[unsafe(no_mangle)]
fn main() -> isize {
    println!("Try to access privileged CSR in U mode");
    println!("Kernel should kill this application");
    // 执行访问特权级CSR的指令出错
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    0
}