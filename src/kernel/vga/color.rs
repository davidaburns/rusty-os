// VgaColor is a 1 byte value (only 4 bits are needed, 16 different color values)
// repr(u8) means the memory layout will be capped at u8 size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGray = 0x07,
    DarkGray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    LightCyan = 0x0B,
    LightRed = 0x0C,
    Pink = 0x0D,
    Yellow = 0x0E,
    White = 0x0F
}

// Debug = Provides a debug representation of the struct for println!()
// Clone = Allows for a deep clone of the struct, supertrait of Copy which means anything with is being cloned must also implement Copy
// Copy = Allows the struct to be a simple bit by bit copy
// PartialEq = Allows for comparison of types that have partial equivalence, structs only equal when all fields are equal
// repr(transparent) means the data layout is the same as a u8, can only be used on structs with a single non-zero sized field

// VgaColorCode takes 2 4bit color values (VgaColor) and OR's them together to make a 1byte vga color code
// first 4 bits = foreground
// second 4 bits = background
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VgaColorCode(u8);

impl VgaColorCode {
    pub fn new(fg: VgaColor, bg: VgaColor) -> VgaColorCode {
        // (bg) 0000 | (fg) 0000 = (full color) 00000000
        VgaColorCode((bg as u8) << 4 | (fg as u8))
    }
}