#![no_std]
#![no_main]

extern crate rs_arm_core;

mod mcg;
pub mod register;
mod vectors;

static RO_DATA: &[u8] = b"Hello world!";
static mut BSS: u8 = 0;
static mut DATA: u8 = 1;

#[no_mangle]
pub fn main() -> ! {
    let _x = RO_DATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    loop {}
}
