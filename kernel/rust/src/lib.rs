#![no_std]

mod dump;
mod hello;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[allow(improper_ctypes_definitions)]
extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}
