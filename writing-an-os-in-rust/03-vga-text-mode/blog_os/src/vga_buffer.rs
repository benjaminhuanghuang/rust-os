use volatile::Volatile;

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
  //chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
  chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
        self.buffer.chars[row][col] = ScreenChar {
          ascii_character: byte,
          color_code,
        };
        self.column_position += 1;
      }
    }
  }

  fn new_line(&mut self) { /* TODO */
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

// For testing
pub fn print_something() {
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
}
