# Heap Allocation

https://os.phil-opp.com/heap-allocation/

## Local and Static Variables

Local variables are stored on the call stack
![](./local-var.png)

Static variables are stored at a fixed memory location separate from the stack.
![](./static-var.png)
Statics live for the complete runtime of the program

## Dynamic Memory

- Local variables only live until the end of the surrounding function or block. This is because they live on the call stack and are destroyed after the surrounding function returns.

- Static variables always live for the complete runtime of the program, so there is no way to reclaim and reuse their memory when they’re no longer needed. Also, they have unclear ownership semantics and are accessible from all functions, so they need to be protected by a Mutex when we want to modify them.

  ![](./dynamic-mem.png)

## Common Errors

- use-after-free vulnerability
- double-free vulnerability

## The Allocator Interface

使用 rust standard lib 自带的 alloc create

```
// in src/lib.rs

extern crate alloc;
```
