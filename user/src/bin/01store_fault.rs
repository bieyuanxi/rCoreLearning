#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;


#[unsafe(no_mangle)]
fn main() ->i32 {
    println!("Into test store_fault, we will insert an invalid store operation");
    println!("Kernel should kill this application");
    // 故意访问了一个非法地址，导致应用和 qemu-riscv64 被 Linux 内核杀死
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}