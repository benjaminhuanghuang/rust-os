# Double Faults

https://os.phil-opp.com/double-fault-exceptions/

## What is a Double Fault?

double fault 在 CPU 调用异常处理函数失败的时候触发的异常
例如，CPU 触发 page fault, 但是 IDT 中却没有对应处理函数的情况

双重异常 Interrupt Vector Number = 8

双重异常在不被处理的情况下会引发致命的三重异常。三重异常不能被捕获，而且会引起大多数硬件的系统复位。

## Triggering a Double fault

```
// in src/main.rs

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    // 这个虚拟地址没有在页表中被映射到物理地址，这会触发一个缺页异常。
    // 而缺页异常的处理函数还没有被定义到 IDT ，因此双重异常被触发了
    unsafe {
      *(0xdeadbeef as *mut u64) = 42;
    };

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}
```

## Double Fault Handler

```
// in src/interrupts.rs

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler); // new
        idt
    };
}

// new
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
```

## Kernel Stack Overflow

guard page 是存在栈底的特定内存页, 用来发现栈溢出
引导程序（bootloader）为内核栈建立了保护页

## Switching Stacks

x86_64 架构可以在异常发生时切换到预定义且已知良好的栈中。这个切换发生在硬件级别，所以它可以在 CPU 压入异常栈帧之前完成。

在 32 位模式下，TSS (Task State Segment)被用来保存任务（task）相关的各种信息（例如寄存器的状态），包括硬件上下文切换（hardware context switching）等。

在 x86_64 架构下，TSS 不再保存任何关于任务（task）的信息。
