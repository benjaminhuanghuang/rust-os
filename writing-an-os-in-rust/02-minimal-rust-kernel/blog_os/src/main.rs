//#![feature(asm)]  stable since 1.59.0 and no longer require
#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点

use core::panic::PanicInfo;


static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in HELLO.iter().enumerate() {
      unsafe {
          *vga_buffer.offset(i as isize * 2) = byte;    // char
          *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color 
      }
  }

  loop {}
}

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}