use core::fmt;

use super::color::VgaColorCode;
use super::buffer::{VgaChar, VgaBuffer, VGA_BUFFER_WIDTH, VGA_BUFFER_HEIGHT};

pub struct VgaWriter {
    pub column_pos: usize,
    pub color: VgaColorCode,
    pub buffer: &'static mut VgaBuffer // 'static lifetime identifier means that the reference is valid the entire lifespan of the program
}

impl VgaWriter {
    // Writes a single byte to the vga buffer, incrementing column position
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => { self.new_line() },
            byte => {
                // Wrap to the next line first before writting the byte
                if self.column_pos >= VGA_BUFFER_WIDTH {
                    self.new_line()
                }

                // Obtain the row, col and color of the character
                let row = VGA_BUFFER_HEIGHT - 1;
                let col = self.column_pos;
                let color = self.color;

                // Store the character info in the buffer
                self.buffer.bytes[row][col].write(VgaChar {
                    ascii: byte,
                    color: color
                });

                // Adjust the column position to the next output position
                self.column_pos += 1;
            }
        }
    }

    // Write a string to the vga output buffer
    // Iterates over the raw bytes of the string and calls write_byte() to write the individual byte
    pub fn write_str(&mut self, s: &str) {
        // Print only prinable ASCII bytes or newline byte, else print an 'unknown' character
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // ASCII Byte
                _ => self.write_byte(0xfe) // Unknown byte
            }
        }
    }

    // Loops through the buffer and shifts each character to a row above it
    fn new_line(&mut self) {
       for row in 1..VGA_BUFFER_HEIGHT {
           for col in 0..VGA_BUFFER_WIDTH {
               let character = self.buffer.bytes[row][col].read();
               self.buffer.bytes[row - 1][col].write(character);
           }
       }

       self.clear_row(VGA_BUFFER_HEIGHT - 1);
       self.column_pos = 0;
    }

    // Writes out a blank space byte for each column in the specified row of the buffer "clearing" that row
    fn clear_row(&mut self, row: usize) {
        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer.bytes[row][col].write(VgaChar {
                ascii: b' ',
                color: self.color
            });
        }
    }
}

// Implement the fmt::Write trait to take advantage of rust string formatting
impl fmt::Write for VgaWriter {
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_byte(c as u8);
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}