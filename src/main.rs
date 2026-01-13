#![no_std]
#![no_main]

use cortex_m::asm;
use defmt::println;
// pick a panicking behavior
// use panic_rtt_target as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _; // panic handler for `defmt`

#[entry]
fn main() -> ! {
    //rtt_init_print!(rtt_target::ChannelMode::NoBlockSkip);
    println!("Hello, world!");
    loop {
        asm::nop();
    }
}
