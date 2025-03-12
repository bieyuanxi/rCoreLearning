mod context;

pub use context::TrapContext;

use core::arch::global_asm;

use riscv::{
    interrupt::{Exception, Interrupt, Trap},
    register::{
        scause, stval,
        stvec::{self, Stvec, TrapMode},
    },
};

use crate::{batch::run_next_app, syscall::syscall};

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" {
        fn __alltraps();
    }
    unsafe {
        let mut v = Stvec::from_bits(0);
        v.set_address(__alltraps as usize);
        v.set_trap_mode(TrapMode::Direct);
        stvec::write(v);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    let cause: Trap<Interrupt, Exception> = scause.cause().try_into().unwrap();
    match cause {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
