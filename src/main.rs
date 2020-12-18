#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;

//semi hosting
use cortex_m_semihosting::hprintln;


#[entry]
fn main() -> ! {
    hprintln!("Hello world from logging").unwrap();

    loop {
        // your code goes here
    }
}
