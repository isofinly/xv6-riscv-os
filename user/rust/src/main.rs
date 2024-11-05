#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" {
    fn write(fd: i32, buf: *const u8, n: i32) -> i32;
    fn exit(code: i32) -> !;
    fn printf(format: *const u8, ...) -> i32;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { exit(1) }
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let msg = b"Hello from Rust!\n";
    let ptr = msg.as_ptr();

    unsafe {
        // printf(ptr);
        printf(b"%s\0".as_ptr(), ptr);
        write(1, msg.as_ptr(), msg.len() as i32);
        exit(0)
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { exit(main()) }
}
