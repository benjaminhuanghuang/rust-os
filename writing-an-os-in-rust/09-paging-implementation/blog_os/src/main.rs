#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use x86_64::structures::paging::PageTable;

/*
  defines the real lower level _start entry point for us
*/
entry_point!(kernel_main);

// called by bootloader
fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use blog_os::memory::active_level_4_table;
  use blog_os::memory::translate_addr;
  use x86_64::VirtAddr;
  println!("Hello World{}", "!");
  blog_os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

  let addresses = [
    // the identity-mapped vga buffer page
    0xb8000,
    // some code page
    0x201008,
    // some stack page
    0x0100_0020_1a10,
    // virtual address mapped to physical address 0
    boot_info.physical_memory_offset,
  ];

  for &address in &addresses {
    let virt = VirtAddr::new(address);
    let phys = unsafe { translate_addr(virt, phys_mem_offset) };
    println!("{:?} -> {:?}", virt, phys);
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
