#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// custom test FWK create test_main()
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;
use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  #[cfg(test)]
  test_main();

  loop {}
}

#[test_case]
fn trivial_assertion() {
  serial_print!("trivial assertion... ");
  assert_eq!(1, 1);
  serial_println!("[ok]");
}
