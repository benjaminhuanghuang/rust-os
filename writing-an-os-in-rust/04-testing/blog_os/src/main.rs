#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// custom test frame create test_main()
#![reexport_test_harness_main = "test_main"]

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
  println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
}

#[test_case]
fn trivial_assertion() {
  assert_eq!(1, 1);
}
