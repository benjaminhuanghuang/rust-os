
在默认情况下，所有的Rust包（crate）都会链接标准库（standard library），而标准库依赖于操作系统功能，如线程、文件系统、网络。
比如 println!将会向标准输出（standard output）打印字符，它依赖于特殊的文件描述符；这个特性是由操作系统提供的。

标准库还与Rust的C语言标准库实现库（libc）相关联，

为了用Rust编写一个操作系统内核，需要创建独立于操作系统可执行程序。这样的可执行程序常被称作独立式可执行程序（freestanding executable）或裸机程序(bare-metal executable)。




首先在 os 目录下新建 .cargo 目录，并在这个目录下创建 config 文件，并在里面输入如下内容

```
# os/.cargo/config
[build]
target = "riscv64gc-unknown-none-elf"
```
这会对于 Cargo 工具在 os 目录下的行为进行调整：现在默认会使用 riscv64gc 作为目标平台而不是原先的默认 x86_64-unknown-linux-gnu。事实上，这是一种编译器运行的平台（x86_64）与可执行文件运行的目标平台（riscv-64）不同的情况。