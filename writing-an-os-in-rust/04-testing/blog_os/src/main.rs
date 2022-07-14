#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// custom test FWK create test_main()
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;
// use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  #[cfg(test)]
  test_main();

  loop {}
}
/// This function is called on panic.
//#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
  //println!("Running {} tests", tests.len());
  // print test result to host console
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
  /// new
  exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
  serial_print!("trivial assertion... ");
  assert_eq!(1, 1);
  serial_println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
  use x86_64::instructions::port::Port;

  unsafe {
    // 写端口 0xf4, I/O size 为 4(u32)
    let mut port = Port::new(0xf4);
    port.write(exit_code as u32);
  }
}
