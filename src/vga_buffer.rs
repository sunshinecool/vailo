#[allow(dead_code)] // allow unused code
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // inherit in rust is derive
#[repr(u8)] // represent the enum as u8
pub enum Color {
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

/*
 * repr(transparent) ->
 * This can only be used on structs with a single non-zero-sized field 
 * (there may be additional zero-sized fields). 
 * The effect is that the layout and ABI of the whole struct is 
 * guaranteed to be the same as that one field.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/*
 * repr(C) ->
 * This is the most important repr. It has fairly simple intent: do what C does. 
 * The order, size, and alignment of fields is exactly what you would expect 
 * from C or C++. Any type you expect to pass through an FFI boundary should have 
 * repr(C), as C is the lingua-franca of the programming world. 
 * This is also necessary to soundly do more elaborate tricks with data 
 * layout such as reinterpreting values as a different type.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_pos: usize,
    row_pos: usize,
    color: ColorCode,
    /* 'static specifies that the lifetime of the 
     * variable is same as that of the program
     */
    buffer: &'static mut Buffer, 
}

impl Writer {

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color: self.color,
        }; 

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn new_line(&mut self) {
        
        if self.row_pos + 1 == BUFFER_HEIGHT {
            for x in 1..BUFFER_HEIGHT {
                for y in 0..BUFFER_WIDTH {
                    let ch = self.buffer.chars[x][y].read();
                    self.buffer.chars[x-1][y].write(ch);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        } else {
            self.row_pos += 1
        }

        self.col_pos = 0;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[self.row_pos][self.col_pos].write(ScreenChar {
                    ascii_character: byte,
                    color: self.color
                });

                self.col_pos += 1;

            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
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

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        row_pos: 0,
        color: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
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

