#![no_std]
// 只有在集成测试的时候lib才会生成test_main
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

pub trait Testable {
  fn run(&self) -> ();
}
// Insert Printing for every test, call test.run() in test_runner
impl<T> Testable for T
where
  T: Fn(),
{
  fn run(&self) {
    serial_print!("{}...\t", core::any::type_name::<T>());
    self();
    serial_println!("[ok]");
  }
}

// #[cfg(test)] // 去掉 #[cfg(test)], 使得 integration 也可以使用 test_runner
pub fn test_runner(tests: &[&dyn Testable]) {
  // print test result to host console
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test.run(); // Testable
  }
  exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
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

pub fn hlt_loop() -> ! {
  loop {
    x86_64::instructions::hlt();
  }
}

// 以便os, test 公用代码
pub fn init() {
  gdt::init();
  interrupts::init_idt();
  unsafe { interrupts::PICS.lock().initialize() };
  //
  x86_64::instructions::interrupts::enable();
}

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
  // like before
  init();
  test_main();
  hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  test_panic_handler(info)
}
