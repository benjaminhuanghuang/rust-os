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

## volatile。

告诉编译器写入有副作用，不应该被优化掉。

```
# in Cargo.toml
[dependencies]
volatile = "0.2.6"
```
