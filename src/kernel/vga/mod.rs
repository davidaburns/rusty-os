pub mod buffer;
pub mod color;
pub mod writer;

use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

use buffer::VgaBuffer; 
use writer::VgaWriter;
use color::{VgaColor, VgaColorCode};

// A globally static VgaWritter open to the entire program
// Mutex<> makes sure that only one thread is accessing the writter at a time while maintaining call order (Not sure if that last part is true)
#[doc(hidden)]
lazy_static! {
    static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        column_pos: 0,
        color: VgaColorCode::new(VgaColor::Yellow, VgaColor::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) }
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

// Macros that allow the user to utilize variable arguments, formatting them and writting them utilizing the global vga writter
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}