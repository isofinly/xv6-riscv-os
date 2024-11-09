use crate::printf;

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let msg = b"Hello from Kernel Rust!\n\0";
    unsafe {
        printf(msg.as_ptr());
    }
    0
}
