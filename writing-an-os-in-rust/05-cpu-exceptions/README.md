# CPU Exceptions

https://os.phil-opp.com/cpu-exceptions/

When an exception occurs, the CPU interrupts its current work and immediately calls a specific exception handler function

处理异常需要 setup interrupt descriptor table that provides handler functions

OS 可以处理断点`breakpoint exceptions(INT3)` 然后继续执行

## Interrupt Descriptor Table (IDT)

Each entry 16-bytes

When an exception occurs, the CPU roughly does the following:

1. Push some registers on the stack, including the instruction pointer and the RFLAGS register. (We will use these values later in this post.)
2. Read the corresponding entry from the Interrupt Descriptor Table (IDT).
3. Check if the entry is present. Raise a double fault if not.
4. Disable hardware interrupts if the entry is an interrupt gate (bit 40 not set).
5. Load the specified GDT selector into the CS segment.
6. Jump to the specified handler function.

## The Interrupt Calling Convention

Major difference between exceptions and function calls:
函数调用是被编译器生成的 call 指令发起的，而异常可以发生在所有指令的执行过程中

C 语言调用约定明确规定了下面的 preserved and scratch 寄存器：

- preserved 寄存器
  rbp, rbx, rsp, r12, r13, r14, r15
- scratch 寄存器
  rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11

x86-interrupt Calling Convention 需要保存所有用到的寄存器

## 实现

当用户设置了断点，调试程序会使用 int3 指令覆盖对应位置的指令，当 CPU 执行到这一位置的时候会抛出断点异常。当用户希望继续执行程序时，调试程序将 int3 指令替换回原来的指令并继续执行
need

```
#![feature(abi_x86_interrupt)]
```

注意 IDT 生命周期要 是 static 的
因为 OS 本身还没有 heap 的概念, 因此也不能用 Box
