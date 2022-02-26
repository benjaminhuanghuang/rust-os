
# bare-metal executable

## reference 
https://github.com/rustcc/writing-an-os-in-rust/blob/master/01-freestanding-rust-binary.md


不能使用Rust标准库的大部分；
但很多Rust特性依然可以使用。比如迭代器、闭包、模式匹配、Option、Result、字符串格式化，当然还有所有权系统。
这些功能让我们能够编写表达性强、高层抽象的操作系统，而无需操心未定义行为和内存安全。


为了用Rust编写一个操作系统内核，需要独立于操作系统，创建一个可执行程序。
这样的可执行程序常被称作独立式可执行程序（freestanding executable）或裸机程序(bare-metal executable)。


## 禁用标准库
在默认情况下，所有的Rust包（crate）都会链接标准库（standard library），而标准库依赖于操作系统功能，如线程、文件系统、网络。标准库还与Rust的C语言标准库实现库（libc）相关联，它也是和操作系统紧密交互的。编写操作系统，就可以不使用任何与操作系统相关的库,
因此必须使用 no_std 禁用标准库自动引用（automatic inclusion）。 

println!宏是标准库的一部分，也不能使用

```
#![no_std]
fn main() {
   
}
```
panic_handler属性被用于定义一个函数；在程序panic时，这个函数将会被调用。标准库中提供了自己的panic处理函数，但在no_std环境中，需要定义自己的panic处理函数：

## 禁用栈展开
在其它一些情况下，栈展开不是迫切需求的功能；因此，Rust提供了panic时中止（abort on panic）的选项。这个选项能禁用栈展开相关的标志信息生成，也因此能缩小生成的二进制程序的长度。有许多方式能打开这个选项，最简单的方式是把下面的几行设置代码加入
Cargo.toml
```
  [profile.dev]
  panic = "abort"

  [profile.release]
  panic = "abort"
```


## start语言项
一个典型的使用标准库的Rust程序，它的运行将从名为crt0的运行时库开始。crt0意为C runtime zero，它能建立一个适合运行C语言程序的环境，这包含了栈的创建和可执行程序参数的传入。
之后，这个运行时库会调用Rust的运行时入口点，这个入口点被称作start语言项（"start" language item）。
Rust只拥有一个极小的运行时，它只拥有较少的功能，如爆栈检测和打印（stack trace）。之后，运行时将会调用main函数。

独立式可执行程序不能访问Rust运行时或crt0库，所以需要定义自己的入口点。实现一个start语言项并不能解决问题，因为这之后程序依然要求crt0库。所以，我们要做的是，直接重写整个crt0库和它定义的入口点。


## 重写入口点
要告诉Rust编译器不使用预定义的入口点，可以添加#![no_main]属性。

没有Rust底层已有的运行时调用main函数。需要自定义操作系统的入口点，_start函数

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
使用no_mangle标记这个函数，来对它禁用名称重整（name mangling）——这确保Rust编译器输出一个名为_start的函数；否则，编译器可能最终生成名为_ZN3blog_os4_start7hb173fedf945531caE的函数，无法让链接器正确辨别。

将函数标记为extern "C"，告诉编译器这个函数应当使用C语言的调用约定，而不是Rust语言的调用约定。函数名为_start，是因为大多数系统默认使用这个名字作为入口点名称。

## build
安装编译环境：
```
rustup target add thumbv7em-none-eabihf
```

build
```
cargo build --target thumbv7em-none-eabihf
```

