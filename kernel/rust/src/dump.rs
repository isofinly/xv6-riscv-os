use crate::printf;

#[repr(C)]
pub struct TrapFrame {
    pub kernel_satp: u64,   // 0
    pub kernel_sp: u64,     // 8
    pub kernel_trap: u64,   // 16
    pub epc: u64,           // 24
    pub kernel_hartid: u64, // 32
    pub ra: u64,            // 40
    pub sp: u64,            // 48
    pub gp: u64,            // 56
    pub tp: u64,            // 64
    pub t0: u64,            // 72
    pub t1: u64,            // 80
    pub t2: u64,            // 88
    pub s0: u64,            // 96
    pub s1: u64,            // 104
    pub a0: u64,            // 112
    pub a1: u64,            // 120
    pub a2: u64,            // 128
    pub a3: u64,            // 136
    pub a4: u64,            // 144
    pub a5: u64,            // 152
    pub a6: u64,            // 160
    pub a7: u64,            // 168
    pub s2: u64,            // 176
    pub s3: u64,            // 184
    pub s4: u64,            // 192
    pub s5: u64,            // 200
    pub s6: u64,            // 208
    pub s7: u64,            // 216
    pub s8: u64,            // 224
    pub s9: u64,            // 232
    pub s10: u64,           // 240
    pub s11: u64,           // 248
    pub t3: u64,            // 256
    pub t4: u64,            // 264
    pub t5: u64,            // 272
    pub t6: u64,            // 280
}

#[repr(C)]
pub struct SpinLock {
    locked: u32,
    name: *const u8,
    cpu: *mut u64,
}

#[repr(C)]
pub struct Proc {
    pub lock: SpinLock,            // spinlock
    pub state: i32,                // Process state
    pub chan: *mut u64,            // If non-zero, sleeping on chan
    pub killed: i32,               // If non-zero, have been killed
    pub xstate: i32,               // Exit status to be returned to parent's wait
    pub pid: i32,                  // Process ID
    pub parent: *mut Proc,         // Parent process
    pub kstack: u64,               // Virtual address of kernel stack
    pub sz: u64,                   // Size of process memory (bytes)
    pub pagetable: u64,            // User page table
    pub trapframe: *mut TrapFrame, // data page for trampoline.S
    pub context: u64,              // swtch() here to run process
    pub ofile: [u64; 16],          // Open files
    pub cwd: u64,                  // Current directory
    pub name: [u8; 16],            // Process name (debugging)
}

extern "C" {
    pub fn myproc() -> *mut Proc;
}

#[no_mangle]
pub extern "C" fn dump() -> i32 {
    unsafe {
        let p = myproc();
        if p.is_null() {
            return -1;
        }

        let trapframe = (*p).trapframe;

        printf(b"s2 = %d\n\0".as_ptr(), (*trapframe).s2 as i32);
        printf(b"s3 = %d\n\0".as_ptr(), (*trapframe).s3 as i32);
        printf(b"s4 = %d\n\0".as_ptr(), (*trapframe).s4 as i32);
        printf(b"s5 = %d\n\0".as_ptr(), (*trapframe).s5 as i32);
        printf(b"s6 = %d\n\0".as_ptr(), (*trapframe).s6 as i32);
        printf(b"s7 = %d\n\0".as_ptr(), (*trapframe).s7 as i32);
        printf(b"s8 = %d\n\0".as_ptr(), (*trapframe).s8 as i32);
        printf(b"s9 = %d\n\0".as_ptr(), (*trapframe).s9 as i32);
        printf(b"s10 = %d\n\0".as_ptr(), (*trapframe).s10 as i32);
        printf(b"s11 = %d\n\0".as_ptr(), (*trapframe).s11 as i32);
    }
    0
}
