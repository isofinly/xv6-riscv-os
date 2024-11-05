#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" {
    fn pipe(p: *mut [i32; 2]) -> i32;
    fn fork() -> i32;
    fn read(fd: i32, buf: *mut u8, n: i32) -> i32;
    fn write(fd: i32, buf: *const u8, n: i32) -> i32;
    fn exit(code: i32) -> !;
    fn getpid() -> i32;
    fn printf(fmt: *const u8, ...) -> i32;
    fn close(fd: i32) -> i32;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { exit(1) }
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let mut parent_to_child = [0i32; 2];
    let mut child_to_parent = [0i32; 2];

    unsafe {
        // Create two pipes
        if pipe(&mut parent_to_child as *mut [i32; 2]) < 0
            || pipe(&mut child_to_parent as *mut [i32; 2]) < 0
        {
            exit(1);
        }

        match fork() {
            0 => {
                // Child process
                // Close unused pipe ends
                close(parent_to_child[1]); // Close write end of parent->child
                close(child_to_parent[0]); // Close read end of child->parent

                let mut buf = [0u8; 5];
                read(parent_to_child[0], buf.as_mut_ptr(), 4);

                let pid = getpid();
                printf(b"%d: got %s\n\0".as_ptr(), pid, b"ping\0".as_ptr());

                write(child_to_parent[1], b"pong\0".as_ptr(), 4);

                close(parent_to_child[0]);
                close(child_to_parent[1]);
                exit(0);
            }
            pid if pid > 0 => {
                // Parent process
                // Close unused pipe ends
                close(parent_to_child[0]); // Close read end of parent->child
                close(child_to_parent[1]); // Close write end of child->parent

                write(parent_to_child[1], b"ping\0".as_ptr(), 4);

                let mut buf = [0u8; 5];
                read(child_to_parent[0], buf.as_mut_ptr(), 4);

                let pid = getpid();
                printf(b"%d: got %s\n\0".as_ptr(), pid, b"pong\0".as_ptr());

                close(parent_to_child[1]);
                close(child_to_parent[0]);
                exit(0);
            }
            _ => {
                // Fork failed
                exit(1);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { exit(main()) }
}
