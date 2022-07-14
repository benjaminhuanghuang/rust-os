# Testing

https://os.phil-opp.com/testing/

https://github.com/phil-opp/blog_os/tree/post-04

标准的 test crate 依赖标准库, 因为 os 不使用 Rust 标准库 , 无法使用 test

## Custom Test Framworks

custom_test_frameworks 的 工作原理是收集所有标注了 #[test_case]属性的函数, 将这个测试函数的列表作为参数传递给用户指定的 runner 函数

自定义测试框架会生成一个 main 函数来调用 test_runner，但是由于使用了 #[no_main]并提供了指定 \_start 为入口点，所以这个 main 函数就被忽略了。

需要通过 reexport_test_harness_main 属性来将自定义测试框架生成的函数的名称更改为与 main 不同的名称。然后在我们的 \_start 函数里调用这个重命名的函数

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

cargo xtest
```

## 退出 Qemu

在执行完 tests 后， test_runner 会将结果返回给 test_main 函数，而这个函数又返回到 \_start 入口点函数——这样我们就进入了一个死循环，因为入口点函数是不允许返回的。这将导致一个问题：我们希望 cargo xtest 在所有的测试运行完毕后，才返回并退出。
QEMU 支持一种名为 isa-debug-exit 的特殊设备，它提供了一种从客户系统（guest system）里退出 QEMU 的简单方式

```
# in Cargo.toml

[package.metadata.bootimage]
test-args = ["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]
```
