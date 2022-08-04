#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

/*
  defines the real lower level _start entry point for us
*/
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  println!("Hello World{}", "!");

  blog_os::init();

  // Cause page fault
  let ptr = 0xdeadbeaf as *mut u32;
  unsafe {
    *ptr = 42;
  }

  #[cfg(test)]
  test_main();

  // 表明异常被catch住, 程序没有崩溃
  println!("It did not crash!");
  blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  blog_os::test_panic_handler(info)
}
