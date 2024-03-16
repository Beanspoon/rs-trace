#![no_std]
#![no_main]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}