// Panic Handler for 'cargo test'
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use super::qemu::{QemuExitCode, qemu_exit};

    qemu_serial_println!("[failed]\n");
    qemu_serial_println!("Error: {}\n", info);

    qemu_exit(QemuExitCode::Failed);
    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl <T> Testable for T 
where 
    T: Fn() 
{
    fn run(&self) {
        qemu_serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        qemu_serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn runner(tests: &[&dyn Testable]) {
    use super::qemu::{QemuExitCode, qemu_exit};

    qemu_serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    qemu_exit(QemuExitCode::Success);
}

#[test_case]
fn test_assertion() {
    assert_eq!(1, 1);
}