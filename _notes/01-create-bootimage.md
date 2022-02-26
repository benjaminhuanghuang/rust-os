
## BIOS 启动

电脑启动时，主板上BIO将被加载。 BIOS固件将会上电自检、初始化硬件，然后它将寻找一个可引导的存储介质。

如果找到了，那电脑的控制权将被转交给引导程序（bootloader）：一段存储在存储介质的开头的、512字节长度的程序片段。大多数的引导程序长度都大于512字节——所以通常情况下，引导程序都被切分为一段优先启动、长度不超过512字节、存储在介质开头的第一阶段引导程序（first stage bootloader），和一段随后由其加载的、长度可能较长、存储在其它位置的第二阶段引导程序（second stage bootloader）。

引导程序必须决定内核的位置，并将内核加载到内存。引导程序还需要将CPU从16位的实模式，先切换到32位的保护模式（protected mode），最终切换到64位的长模式（long mode）：此时，所有的64位寄存器和整个主内存（main memory）才能被访问。

引导程序的第三个作用，是从BIOS查询特定的信息，并将其传递到内核；如查询和传递内存映射表（memory map）。


## Build target
在默认情况下，cargo会为特定的宿主系统（host system）构建源码

内核应该被编译为一个特定的目标系统（target system）配置文件为：

build config: x86_64-my_os.json
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
  
  "panic-strategy": "abort",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float"
}
```

编译相关配置
```
"linker-flavor": "ld.lld",
"linker": "rust-lld",
```
不能使用平台默认提供的链接器，因为它可能不支持Linux目标系统。为了链接内核需要使用跨平台的LLD链接器（LLD linker），它是和Rust打包发布的

```
"panic-strategy": "abort",
```
编译目标不支持panic时的栈展开（stack unwinding），所以我们选择直接在panic时中止（abort on panic）。这和在Cargo.toml文件中添加panic = "abort"选项的作用是相同的

```
  "disable-redzone": true,
```
编写一个内核应该同时处理中断。要安全地实现这一点，必须禁用一个与红区（redzone）有关的栈指针优化：因为此时，这个优化可能会导致栈被破坏

```
"features": "-mmx,-sse,+soft-float",
```
features配置项被用来启用或禁用某个目标CPU特征（CPU feature）。通过在它们前面添加-号，将mmx和sse特征禁用；添加前缀+号启用了soft-float特征。

## Build tools
为了搭建一个操作系统，需要一些只有nightly会提供的实验性功能，所以需要安装一个nightly版本的Rust。
```
rustup override add nightly
```
此命令选择在当前目录使用nightly版本的Rust

通常状况下，core库以预编译库（precompiled library）的形式与Rust编译器一同发布——这时，core库只对支持的宿主系统有效，而我们自定义的目标系统无效。如果我们想为其它系统编译代码，我们需要为这些系统重新编译整个core库, 因此需要按装cargo-xbuild, 这个工具封装了cargo build

```
  cargo install cargo-xbuild
```

编译
```
cargo xbuild --target x86_64-my_os.json
```

## Create image
将可执行程序转换为可引导的映像（bootable disk image），需要把它和引导程序链接。
这里，引导程序将负责初始化CPU并加载我们的内核。


此处不用自己编写引导程序，而是使用已有的bootloader包；
这个包基于Rust代码和内联汇编，实现了一个五脏俱全的BIOS引导程序。为了用它启动我们的内核，需要将它添加为一个依赖项，在Cargo.toml中添加：
```
# in Cargo.toml

[dependencies]
bootloader = "0.6.0"
```
只添加引导程序为依赖项，并不足以创建一个可引导的磁盘映像；我们还需要内核编译完成之后，将内核和引导程序组合在一起。

使用bootimage将编译好的内核与引导程序组合在一起，最终创建一个能够引导的磁盘映像
```
cargo install bootimage --version "^0.7.3"
```
为了运行bootimage以及编译引导程序，我们需要安装rustup模块llvm-tools-preview——我们可以使用
```
rustup component add llvm-tools-preview
```

build
```
cargo bootimage --target x86_64-my_os.json
```

bootimage工具执行了三个步骤
- 编译我们的内核为一个ELF（Executable and Linkable Format）文件；
- 编译引导程序为独立的可执行文件；
- 将内核ELF文件按字节拼接（append by bytes）到引导程序的末端。


## Run the OS
在QEMU中启动 boot image：
```
  qemu-system-x86_64 -drive format=raw,file=bootimage-my_os.bin
```
当机器启动时，引导程序将会读取并解析拼接在其后的ELF文件。然后把program segments 映射到分页表（page table）中的虚拟地址（virtual address），清零BSS段（BSS segment），还将创建一个栈。
最终它将读取入口点地址（entry point address）_start函数的位置—, 并跳转到这个位置。