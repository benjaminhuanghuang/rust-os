use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// use the spinning Mutex to add safe interior mutability to our static WRITER:
lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
}

#[allow(dead_code)] // some enums are dead code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//repr(u8) 表示enum variant is stored as an u8. 实际上 4 位就足够了，但 Rust 没有u4类型。
#[repr(u8)]
pub enum Color {
  // C-like enum
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// 保证内存布局与其中的单字段相同
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // C-like 保证字段顺序
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
  //chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
  // 避免编译器优化
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
  column_position: usize,
  color_code: ColorCode,
  // reference life time is whole program run time
  buffer: &'static mut Buffer,
}

impl Writer {
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      // new line
      b'\n' => self.new_line(),
      // normal char
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        // 因为只有写操作,没有读操作, 直接赋值有可能被编译器优化掉
        //self.buffer.chars[row][col] = ScreenChar {
        self.buffer.chars[row][col].write(ScreenChar {
          ascii_character: byte,
          color_code,
        });
        self.column_position += 1;
      }
    }
  }

  fn new_line(&mut self) {
    //向上滚动
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }
    self.clear_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
  }

  fn clear_row(&mut self, row: usize) {
    //
    let blank = ScreenChar {
      ascii_character: b' ',
      color_code: self.color_code,
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }

  pub fn write_string(&mut self, s: &str) {
    // writh bytes in &str
    for byte in s.bytes() {
      match byte {
        // printable ASCII byte or newline
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // not part of printable ASCII range
        _ => self.write_byte(0xfe),
      }
    }
  }
}

// For testing, called in _start
pub fn print_something() {
  use core::fmt::Write;

  let mut writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    // cast 0xb8000 as an mutable raw poiniter
    // covert it to a mutalbe reference by dereferening it through *
    // borrow it through &mut
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  };

  writer.write_byte(b'H');
  writer.write_string("ello ");
  writer.write_string("Wörld!");

  write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}

// print Write like int or float
impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for byte in s.bytes() {
      self.write_byte(byte)
    }
    Ok(())
  }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use core::fmt::Write;
  WRITER.lock().write_fmt(args).unwrap();
}

//------------
#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_println_simple() {
  println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
  // ensure that no panic occurs even if many lines are printed
  for _ in 0..200 {
    println!("test_println_many output");
  }
}

#[test_case]
fn test_println_output() {
  // verify that the printed lines really appear on the screen
  let s = "Some test string that fits on a single line";
  println!("{}", s);
  for (i, c) in s.chars().enumerate() {
    let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
    assert_eq!(char::from(screen_char.ascii_character), c);
  }
}
