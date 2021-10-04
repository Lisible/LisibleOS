#![feature(asm)]
#![no_std]
#![no_main]

use crate::terminal::Terminal;
use core::panic::PanicInfo;

mod io;
mod terminal;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut terminal = Terminal::new();
    terminal.clear();
    terminal.put_string(b"LisibleOS\n");
    terminal.put_string(b"> ");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
