use core::fmt;
use core::fmt::Write;

use x86_64::instructions::port::Port;
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn qemu_exit(code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(code as u32);
    }
}


// QEMU Serial Interface
lazy_static! {
    static ref QEMU_SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();

        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _serial_print(args: fmt::Arguments) {
    QEMU_SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
macro_rules! qemu_serial_print {
    ($($arg:tt)*) => {
        $crate::kernel::qemu::_serial_print(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! qemu_serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::qemu_serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::qemu_serial_print!(
        concat!($fmt, "\n"), $($arg)*
    ));
}