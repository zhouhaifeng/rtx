//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(asm)]`  
//!   内嵌汇编
//#![feature(asm)]
//!
//! - `#![feature(global_asm)]`
//!   内嵌整个汇编文件
//#![feature(global_asm)]

//#![feature(lang_items)]

use core::panic::PanicInfo;
//use std::fmt;
//use std::io::{stdout, Write};

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        core::arch::asm!("ecall",
            inout("x10") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
        );
    }
    ret
}

pub fn console_putchar(ch: usize) -> usize {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, ch, 0, 0)
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0)
}

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
//#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 在屏幕上输出 "OK\n" ，随后进入死循环
    console_putchar('O');
    console_putchar('K');
    console_putchar('\n');

    loop {}
}
