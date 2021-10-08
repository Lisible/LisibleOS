#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::terminal::Terminal;

mod arch;
mod serial;
mod terminal;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut terminal = Terminal::new();
    terminal.clear();
    if let Err(_) = serial::initialize_serial_port() {
        terminal.put_string(b"Serial port initialization failed");
    } else {
        terminal.put_string(b"Serial port initialization succeeded");
    }

    serial::write_bytes(b"This string was sent through the serial port!!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
