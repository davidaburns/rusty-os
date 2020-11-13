use volatile::Volatile;
use super::color::VgaColorCode;

// Dimensions of the buffer used to calculate when to wrap to next line, and how to clear and output a newline
pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;

// repr(C) means the struct maintains its field ordering the order that it is layed out in code (top -> bottom)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VgaChar {
    pub ascii: u8,
    pub color: VgaColorCode,
}

#[repr(transparent)]
pub struct VgaBuffer {
    pub bytes: [[Volatile<VgaChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}