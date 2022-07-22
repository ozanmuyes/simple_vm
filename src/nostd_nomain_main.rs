#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    match my_main(/* TODO MAYBE params */) {
        Ok(_) => syscall_exit(0),
        Err(exit_code) => syscall_exit(exit_code), // FIXME make use of the `error` to find out exit code (instead of hardcoded `1`)
    }

    loop {}
}

// TODO MAYBE params, return type
fn my_main() -> Result<(), i32> {
    print("Hello world!");

    // ...

    Ok(())
    // Err(42)
}

#[inline(always)]
fn print(message: &str) {
    syscall_write(1, message);
}

// TODO TR aşağıdaki syscall'ları ayrı modüllere çıkar (3 modül = 3 OS)

// #[cfg(target_os = "macos")]
#[inline(never)]
fn syscall_write(fd: i32, buf: &str) {
    let buf_ptr = buf.as_ptr();
    let buf_len = buf.len();

    unsafe {
        asm!(
            "mov rax, 0x2000004",   // syscall write = 0x2000004 on mac
            "syscall",
            in("rdi") fd,
            in("rsi") buf_ptr,      // address of string to output
            in("rdx") buf_len,      // number of bytes
            // out("rax") _, lateout("rdi") _, lateout("rsi") _, lateout("rdx") _
        )
    }
}

#[inline(never)]
fn syscall_exit(code: i32) {
    unsafe {
        asm!(
            "mov rax, 0x2000001",   // syscall exit = 0x2000001 on mac
            "syscall",
            in("rdi") code,
        )
    }
}
