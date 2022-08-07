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
  // new: different imports
  use blog_os::memory;
  use x86_64::{structures::paging::Page, VirtAddr};

  println!("Hello World{}", "!");
  blog_os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = memory::EmptyFrameAllocator;

  // map an unused page
  let page = Page::containing_address(VirtAddr::new(0));
  memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

  // write the string `New!` to the screen through the new mapping
  let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
  unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

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
