// VGA BUFFER ABSTRACTION
// GOAL:
// Writing into the display is a dangerous and an error prone action,
// requiring the use of unsafe blocks of code, manual address conversion, etc.
// This module abstracts away this danger and provides the Global
// WRITER object as an interface for printing into the display.
//
// # Display Format
// The VGA Buffer is a 2D array, typically 25 rows and 80 columns
// Each array entry describes a single character to be displayed
// The format of the entry(in Bits):
// 0-7: ASCII code of the character
// 8-11: Foreground color
// 12-14: Background Color
// 15: Blink
use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Global writer that can be used from anywhere.
//
// Why use lazy_static: Static variable in rust will be initialized at compile
// time, but here we need to convert raw pointer into a reference (the unsafe
// block), which rust can't do at compile time, so we add lazy_static which
// will initialize the static variable lazily aka. at run time when the static
// variable is accessed for the first time, which will avoid the pointer
// conversion.

lazy_static! {
    // We use Mutex(provided by the spin crate, because we can't use stdlib)
    // here to provide the ability to "lock" the WRITER so that we can ensure
    // that the WRITER is only accessed by one processes at any time, (we
    // don't want the buffer to be accessed by different processes at the same
    // time and possibly create data corruption).
    // See:
    // - https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html
    // - https://os.phil-opp.com/vga-text-mode/#spinlocks
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
       column_position: 0,
       color_code: ColorCode::new(Color::Yellow, Color::Black),
       buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // 0xb8000 = the location of VGA buffer
    });
}

// # COLORS=====================================================================
// Defines how the colors will be represented.

// Prevent the compiler from throwing error on unused code, (since not all enum
// will be used)
#[allow(dead_code)]
// Make the Color enum comparable and printable by deriving Clone, Eq... traits
// see: https://doc.rust-lang.org/rust-by-example/trait.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// store the enum as u8 ( Unsigned 8 bit integer)
#[repr(u8)]
// Standard color palette in VGA text mode
pub enum Color {
    Black = 0,
    // 0000 0000
    Blue = 1,
    // 0000 0001
    Green = 2,
    // 0000 0010
    Cyan = 3,
    // 0000 0011
    Red = 4,
    // 0000 0100
    Magenta = 5,
    // 0000 0101
    Brown = 6,
    // 0000 0110
    LightGray = 7,
    // 0000 0111
    DarkGray = 8,
    // 0000 1000 (Byte 4 flips color to light version, eg. Black to DarkGrey)
    LightBlue = 9,
    // 0000 1001
    LightGreen = 10,
    // 0000 1010
    LightCyan = 11,
    // 0000 1011
    LightRed = 12,
    // 0000 1100
    Pink = 13,
    // 0000 1101
    Yellow = 14,
    // 0000 1110
    White = 15,  // 0000 1111
}

// Make the ColorCode comparable and printable by deriving Clone, Eq... traits
// see: https://doc.rust-lang.org/rust-by-example/trait.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// A custom type that contains the full color byte containing background and
// foreground color Bit 9-11 + 12-14
// see: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
struct ColorCode(u8);

// impl is used to define methods.
// see: https://doc.rust-lang.org/rust-by-example/fn/methods.html
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// # TEXT BUFFER ==============================================================
// Struct for holding the character to be displayed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Field ordering in default struct is undefined in rust,
// so we use `repr(c)` to guarantees that the struct field is ordered, just like
// in C.
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    // a 2D array of Volatile<ScreenChar>
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

// # WRITER =================================================================
// Writer is used to abstract away printing into the VGA Buffer.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                //not part of printable ASCII range
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        // The printing goes from bottom to top, inserting a new
        // line means shifting all existing printed char up by one line
        // and writing the new line on the bottom of the screen.
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character)
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        // Replace whole line with whitespace
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// # FORMATTING MACROS SUPPORT ================================================
// Adds support for rust's formatting macros (write! / writeln!)
// ex. (write!(writer, ", some numbers: {} {}", 42, 1.337) will format the string)
// this way it'll be easy to print integer and floats
//
// see: https://doc.rust-lang.orgformate/std/macro.writeln.html

// trait implementation, add a trait and override one of the function
// vvv
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// # PRINTLN MACRO ============================================================
// Create a println macro that can be used anywhere

// #{macro_export} creates a global macro that is accessible from anywhere.
// Macro allows us to abstract code in the syntactic level. It's very powerful
// and complex. It gives us the ability to pattern match based on
// syntax (similar to regex) and expand it into the code that we need.
// see:
// - https://doc.rust-lang.org/1.30.0/book/first-edition/macros.html
// - https://doc.rust-lang.org/1.30.0/reference/macros-by-example.html#macros-by-example
#[macro_export]
// macro_rules acts like a switch case.
macro_rules! println {
    // Below are called rules, "(pattern) => (code expansion)"
    () => (print!("\n"));
    ($($arg:tt)*) =>($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffers::_print(format_args!($($arg)*)));
}

// This function needs to be accessible from outside the module (hence the `pub`),
// but we consider it as a private implementation detail, hence the #[doc(hidden)]
// which will prevent it from showing in a generated doc.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap()
}


// # TEST  --------============================================================
#[cfg(test)]
mod test {
    // import all parent module items
    use super::*;

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Blue, Color::Magenta),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;

        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char())))
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Brown),
        }
    }

    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn write_formatted() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 3 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'c');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i >= BUFFER_HEIGHT - 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
}

#[test]
fn foo() {}
