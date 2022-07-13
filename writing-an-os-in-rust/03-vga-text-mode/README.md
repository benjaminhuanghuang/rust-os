# VAG Text mode

https://os.phil-opp.com/vga-text-mode/

## VGA text buffer

VGA Text Buffer 25 行、80 列 共 2000 个字符单元（character cell）

映射到内存 address 0xb8000。对该地址的读取和写入不会访问 RAM，而是直接访问 VGA 硬件上的文本缓冲区, 只能支持按字节读取

0-7 ASCII code
8-12 front color
12-14 background color
15 blink

## Create vga_buffer

## volatile

程序只是写 buffer, 没有读 buffer, 编译器可能会优化掉写操作

volatile library provides a Volatile wrapper type with read and write methods.
These methods internally use the read_volatile and write_volatile functions
of the core library and thus `guarantee that the reads/writes are not optimized away`.

```
# in Cargo.toml
[dependencies]
volatile = "0.2.6"
```

```
struct Buffer {
  //chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
  // 避免编译器优化
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// 因为只有写操作,没有读操作, 直接赋值有可能被编译器优化掉
self.buffer.chars[row][col].write(ScreenChar {
          ascii_character: byte,
          color_code,
        });
```

## Formatting Macros

实现 core::fmt::Write

## Global interface

希望 writer 可以被全局访问,而不是被作为变量到处传递

lazy_static! 定义的 intance 会在第一次被访问时初始化

```
[dependencies.lazy_static]
version = "1.0"
features = ["spin_no_std"]
```

## Spinlocks

```
# in Cargo.toml
[dependencies]
spin = "0.5.2"
```
