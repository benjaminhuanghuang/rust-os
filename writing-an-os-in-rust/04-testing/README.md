# Testing

https://os.phil-opp.com/testing/

https://github.com/phil-opp/blog_os/tree/post-04

标准的 test library 依赖标准库, 因为 os 不使用 Rust 标准库 , 无法使用 test

## Custom Test Framworks

custom_test_frameworks 的工作原理是收集所有标注了 #[test_case]属性的函数, 将这个测试函数列表作为参数传递给用户指定的 runner 函数

Custom Test Framwork 会生成一个 main 函数来调用 test_runner，但是由于使用了 #[no_main]并提供了指定 \_start 为入口点，所以这个 main 函数就被忽略了。

需要通过 reexport_test_harness_main 属性来将 Custom Test Framwork 生成的 entry point function 更改为与 test_main 。然后在入口函数\_start 里调用这个重命名的函数

```
// in src/main.rs
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
```

Run test

```
rustup override add nightly

cargo test
```

## 退出 Qemu

执行完 tests 后， test_runner 会将结果返回给 test_main 函数，而这个函数又返回到 \_start 入口点函数——这样我们就进入了一个死循环，因为入口点函数是不允许返回的。对于测试自动化, 我们希望 cargo test 在所有的测试运行完毕后可以自动退出。

QEMU 支持一种名为`isa-debug-exit`的特殊设备，它提供了一种从客户系统（guest system）里退出 QEMU 的简单方式

```
# in Cargo.toml

[package.metadata.bootimage]
test-args = ["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]
```

bootimage runner 会在执行 cargo test 的时候把 test-args 传给 QEMU

`isa-debug-exit` 使用了 port-mapped I/O, 参数 iobase=0xf4,iosize=0x04
写入的内容会转化成 Qemu 的 退出状态 `(value << 1) | 1`

这里引入 x86_64 crate, 它提供了对汇编指令的包装

```
# in Cargo.toml

[dependencies]
x86_64 = "0.14.10"
```

使用 x86_64 提供的 Port 写 port

```
// in src/main.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// in src/main.rs

fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    /// new
    exit_qemu(QemuExitCode::Success);
}
```

而 cargo test 会认为 所有非 0 的 返回值都是 error
因此 要设置 bootimage, 自定义 succes 的退出代码

```
# in Cargo.toml

[package.metadata.bootimage]
test-args = [...]
test-success-exit-code = 33         # (0x10 << 1) | 1
```

## Print to test result host console

需要把 os 中产生的测试结果发送到 host console
solution 是 serial port 通信, 使用 crate

```
# in Cargo.toml

[dependencies]
uart_16550 = "0.2.0"
```

实现见 serial.rs, 使用:

```
// in src/main.rs
mod serial;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    […]
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
```

还要通知 bootimage 把 output 从 serial port redirect 到 stdout

```
# in Cargo.toml

[package.metadata.bootimage]
test-args = [
  "-device", "isa-debug-exit,iobase=0xf4,iosize=0x04", "-serial", "stdio"
]
```

还没完, 还要处理 panic 的情况, 正常情况 panic 向 console 输出, test 时向 serial 输出

```
// in src/main.rs

// our existing panic handler
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

```

通过 bootimage 向 qemu 传递参数, 隐藏 qemu 窗口, 这样就可以在没有 GUI 的情况下执行测试

```
# in Cargo.toml

[package.metadata.bootimage]
test-args = [
    "-device", "isa-debug-exit,iobase=0xf4,iosize=0x04", "-serial", "stdio",
    "-display", "none"
]
```

设置超时
If the test does not finish in this time, bootimage is marked as failed and a “Timed Out” error is printed to the console.

```
# in Cargo.toml

[package.metadata.bootimage]
test-timeout = 300          # (in seconds)
```

## Unit Test

```
// Insert Printing for every test, call test.run() in test_runner

pub trait Testable {
  fn run(&self) -> ();
}

impl<T> Testable for T
where
  T: Fn(),
{
  fn run(&self) {
    serial_print!("{}...\t", core::any::type_name::<T>());
    self();
    serial_println!("[ok]");
  }
}
```

修改 test_runner 的传入参数类型 为 Testable 并调用 test.run

```
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
  // print test result to host console
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test.run(); // Testable
  }
  /// new
  exit_qemu(QemuExitCode::Success);
}

```

## Integration Test

All integration tests are their own executables and completely separate from our main.rs.
This means that each test needs to define its own entry point function.

## Refactor

Move code into lib.rs
