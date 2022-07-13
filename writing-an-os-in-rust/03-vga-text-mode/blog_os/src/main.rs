//#![feature(asm)]  stable since 1.59.0 and no longer require
#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
  use core::fmt::Write;
  vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
  write!(
    vga_buffer::WRITER.lock(),
    ", some numbers: {} {}",
    42,
    1.337
  )
  .unwrap();

  loop {}
}

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}