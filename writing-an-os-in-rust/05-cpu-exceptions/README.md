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
