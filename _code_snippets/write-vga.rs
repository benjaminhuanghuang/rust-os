#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte; // char
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color
    }
  }

  loop {}
}
