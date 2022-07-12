https://github.com/rustcc/writing-an-os-in-rust/blob/master/01-freestanding-rust-binary.md


1. create a exe project
```
  cargo new blog_os
```

2. 禁用stdlib，使用no_std属性
```
// main.rs

#![no_std]
fn main() {
}
```
println!会向标准输出（standard output）打印字符，它依赖于特殊的文件描述符；这个特性是由操作系统提供的。



3. 定义自己的panic 处理函数
```
// in main.rs

use core::panic::PanicInfo;

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```
类型为PanicInfo的参数包含了panic发生的文件名、代码行数和可选的错误信息。
这个函数从不返回, 所以他被标记为发散函数 (diverging function)。
发散函数的返回类型称作Never类型("never" type), 记为!

4. 禁用栈展开
在使用标准库的情况下，当panic发生时，Rust将使用栈展开，来运行在栈上活跃的所有变量的析构函数（destructor）——这确保了所有使用的内存都被释放，允许调用程序的父进程（parent thread）捕获panic，处理并继续运行。但是，栈展开是一个复杂的过程，如Linux的libunwind或Windows的结构化异常处理（structured exception handling, SEH），通常需要依赖于操作系统的库；所以要禁止。

```
# 禁用栈展开相关的标志信息生成
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

5. 定义 entry point
使用标准库的Rust程序将从名为crt0(C runtime zero)的运行时库开始运行。

crt0能建立一个适合运行C语言程序的环境，这包含了栈的创建和可执行程序参数的传入。

然后运行时库会调用Rust的运行时entry point，这个入口点被称作start语言项（"start" language item）。Rust只拥有一个极小的运行时，它只拥有较少的功能，如爆栈检测和打印stack trace。

然后，运行时将会调用main函数。

独立式可执行程序不能访问Rust运行时或crt0库，所以需要定义自己的入口点, 重写整个crt0库和它定义的入口点。

```
// 告诉Rust编译器使用预定义的入口点
#![no_main]


#[no_mangle]  // 确保Rust编译器输出一个名为_start的函数, 不会改名字
// extern "C" 告诉编译器这个函数应当使用C语言的调用约定，而不是Rust语言的调用约定。
// 函数名为_start，大多数系统默认使用这个名字作为入口点名称。
pub extern "C" fn _start() -> ! {
    loop {}
}
```


5. 链接
链接器的默认配置假定程序依赖于C语言的运行时环境, 需要告诉链接器禁用C语言运行环境。我们可以选择提供特定的链接器参数（linker argument），也可以选择编译为裸机目标（bare metal target）. 此处选择裸机目标 thumbv7em-none-eabihf

安装编译环境, 为目标下载一个标准库和core库
```
rustup target add thumbv7em-none-eabihf
```
