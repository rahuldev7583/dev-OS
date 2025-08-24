#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();
    println!("Hello world using println");
    panic!("Some panic message");

    // loop {}
}
