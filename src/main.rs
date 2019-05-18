#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

#[entry]
fn main() -> ! {

    // This should never execute
    hprintln!("Program complete.").unwrap();
    loop {}
}
