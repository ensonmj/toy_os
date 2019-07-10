#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::Writer::new();
    writer.write_string("Hello Wörld!\n");

    use core::fmt::Write;
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}