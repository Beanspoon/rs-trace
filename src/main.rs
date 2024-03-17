#![no_std]
#![no_main]

// #[cfg(not(test))]
use core::panic::PanicInfo;

// #[cfg(not(test))]
#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn reset_handler() -> ! {
    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: fn() -> ! = reset_handler;
