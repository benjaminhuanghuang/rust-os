# Hardware Interrupts

https://os.phil-opp.com/hardware-interrupts/

```
                       ____________             _____
  Timer ------------> |            |           |     |
  Keyboard ---------> | Interrupt  |---------> | CPU |
  Other Hardware ---> | Controller |           |_____|
  Etc. -------------> |____________|

```

hardware interrupts occur `asynchronously`
因此，内核中就突然出现了一种并发形式

## The 8259 PIC ( programmable interrupt controller)

在主控制器中，这两个端口分别位于 0x20（命令）和 0x21（数据）。 而在从控制器中，分别是 0xa0（命令）和 0xa1（数据）

不能使用默认的 PIC 配置，因为它将会向 CPU 发送 0-15 范围内的中断类型码

通常会选择范围 32-47 的数字

配置是通过向 PIC 的命令和数据端口写入特殊值来完成的

```
# in Cargo.toml

[dependencies]
pic8259_simple = "0.1.1"
```

```
// in src/interrupts.rs

use pic8259_simple::ChainedPics;
use spin;

// set pics to the range 32–47
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe {
      // because wrong offsets could cause undefined behavior
      ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
      });
```

## Enabling Interrupts

```
// in src/lib.rs

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();     // new
}
```

## Handling Timer interrupts

```
// in src/interrupts.rs

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
```
