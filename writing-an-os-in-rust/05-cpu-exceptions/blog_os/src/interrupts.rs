use crate::println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub fn init_idt() {
  let mut idt = InterruptDescriptorTable::new();
}

// 处理断点异常
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
  println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
