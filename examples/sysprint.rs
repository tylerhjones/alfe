use std::arch::asm;

fn sys_print(msg: &str) {
    // mac only
    unsafe {
        asm!(
            "mov rdx,{0}", // length of msg
            "mov rsi,{1}", // msg
            "mov rdi,1", // 1 = stdout
            "mov rax,0x2000004", // 4 = sys_write ? you have to add 0x20000000 to the syscall number on mac ... why??
            "syscall", // yield to kernel
            in(reg) msg.len(),
            in(reg) msg.as_ptr(),
        );
    }
}

fn main() {
    sys_print("Water your plants, but not too much.");
}