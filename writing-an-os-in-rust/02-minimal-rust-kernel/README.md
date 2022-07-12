
1. 使用 bootimage 准备引导程序
BIOS 会加载引导程序, 
- 引导程序必须决定内核的位置，并将内核加载到内存
- 引导程序将CPU从16位的实模式，先切换到32位protected mode），最终切换到64位long mode, 
- 引导程序会在BIOS查询特定的信息，并将其传递到内核；如查询和传递内存映射表（memory map）。

Multiboot标准 定义了引导程序和操作系统间的统一接口

只需要在内核文件开头，插入被称作Multiboot头（Multiboot header）的数据片段。GRUB这样的bootloader就可以引导这个内核文件

2. 配置 target
在默认情况下，cargo会为特定的宿主系统（host system）构建源码，而内核不应该基于另一个操作系统，内核需要编译为一个特定的target system

通过json 文件描述target system
```
{
  "llvm-target": "x86_64-unknown-none",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "arch": "x86_64",
  "target-endian": "little",
  "target-pointer-width": "64",
  "target-c-int-width": "32",
  "os": "none",
  "executables": true,
  "linker-flavor": "ld.lld",
  "linker": "rust-lld",
  "panic-strategy": "abort",    //编译目标不支持panic时的栈展开（stack unwinding）
  "disable-redzone": true,      //禁用一个与红区（redzone）有关的栈指针优化
  "features": "-mmx,-sse,+soft-float"   // 启用或禁用某个目标CPU特征（CPU feature）
}
```


3. Build, 同时为目标系统重新编译整个core库

首先选择在当前目录使用nightly版本的Rust
```
rustup override add nightly
```

安装 Cargo xbuild
Cargo xbuild 封装了cargo build. 不同的是，它将自动交叉编译core库和一些编译器内建库（compiler built-in libraries）。
```
cargo install cargo-xbuild
```
Cargo xbuild依赖于Rust的源代码；使用rustup component add rust-src来安装源代码。


Build
```
cargo xbuild --target x86_64-blog_os.json
```

为了避免每次使用cargo xbuild时传递--target参数，创建一个名为.cargo/config的cargo配置文件
```
# in .cargo/config
[build]
target = "x86_64-blog_os.json"

[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]
```
直接Build
```
cargo xbuild
```

## 写VGA字符缓冲区（VGA text buffer）
这是一段映射到VGA硬件的特殊内存片段，包含着显示在屏幕上的内容。通常情况下，它能够存储25行、80列共2000个字符单元（character cell）；每个字符单元能够显示一个ASCII字符，也能设置这个字符的前景色（foreground color）和背景色（background color）
这段缓冲区的地址是0xb8000，且每个字符单元包含一个ASCII码字节和一个颜色字节。



## Creat image
在Cargo.toml中添加下面的代码：
```
# in Cargo.toml
[dependencies]
bootloader = "0.9.3"
```
install
```
cargo install bootimage --version "^0.7.3"

rustup component add llvm-tools-preview
```

create image
```
cargo bootimage
```
Image file bootimage-blog_os.bin 会被生成在 target/x86_64-blog_os/debug目录

bootimage的工作:
- 编译内核为一个ELF（Executable and Linkable Format）文件；
- 编译引导程序为独立的可执行文件；
- 将内核ELF文件按字节拼接（append by bytes）到引导程序的末端。


当机器启动时，引导程序将会读取并解析拼接在其后的ELF文件。然后把程序片段映射到分页表（page table）中的虚拟地址（virtual address），清零BSS段（BSS segment），还将创建一个栈。最终它将读取_start函数的位置并跳转


## Run image
```
 qemu-system-x86_64 -drive format=raw,file=bootimage-blog_os.bin
```
