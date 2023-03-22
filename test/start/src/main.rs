// src/main.rs
//#![feature(asm)]
// 在屏幕上输出一个字符，目前我们先不用了解其实现原理

use std::arch::asm;

pub fn console_putchar(ch: u8) {
    let ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        asm!("ecall",
            "={x10}" (ret),
            "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which),
            "memory",
            "volatile",
        );
    }
}
#[no_mangle]
fn main() -> ! {
    // 在屏幕上输出 "OK\n" ，随后进入死循环
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');
    loop {}
}