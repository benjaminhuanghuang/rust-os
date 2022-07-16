# Hardware Interrupts

https://os.phil-opp.com/hardware-interrupts/

```
                       ____________             _____
  Timer ------------> |            |           |     |
  Keyboard ---------> | Interrupt  |---------> | CPU |
  Other Hardware ---> | Controller |           |_____|
  Etc. -------------> |____________|

```

## The 8259 PIC ( programmable interrupt controller)

```
# in Cargo.toml

[dependencies]
pic8259_simple = "0.1.1"
```

```
// in src/interrupts.rs

use pic8259_simple::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
```
