#![no_std]
#![no_main]

use sim::{UartClockSource, SIM};

extern crate rs_arm_core;

mod mcg;
pub mod register;
mod sim;
mod vectors;
// SBR = 26
// OSR = 16
// FRDIV = 1
// baud: 9600
#[no_mangle]
pub fn main() -> ! {
    SIM::access().set_uart_clock_source(UartClockSource::MCGIRClock);

    loop {}
}
