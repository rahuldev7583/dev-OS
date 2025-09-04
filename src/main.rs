#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exist_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exist_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
//static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    //for (i, &byte) in HELLO.iter().enumerate() {
    //  unsafe {
    //    *vga_buffer.offset(i as isize * 2) = byte;
    //  *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //}
    // }
    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap();
    println!("Hello world using println");
    // panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}
