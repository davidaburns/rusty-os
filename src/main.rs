#![allow(dead_code, unused_variables)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::kernel::test::runner)]
#![reexport_test_harness_main = "test_start"]
#![no_std]
#![no_main]

pub mod kernel;

// Panic Handler for Kernel
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_start();

    println!("Hello, World");
    loop {}
}