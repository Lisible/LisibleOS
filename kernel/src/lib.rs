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
    terminal.put_string(b"1 LisibleOS\n");
    terminal.put_string(b"2 > \n");
    terminal.put_string(b"3\n");
    terminal.put_string(b"4\n");
    terminal.put_string(b"5\n");
    terminal.put_string(b"6\n");
    terminal.put_string(b"7\n");
    terminal.put_string(b"8\n");
    terminal.put_string(b"9\n");
    terminal.put_string(b"10\n");
    terminal.put_string(b"11\n");
    terminal.put_string(b"12\n");
    terminal.put_string(b"13\n");
    terminal.put_string(b"14\n");
    terminal.put_string(b"15\n");
    terminal.put_string(b"16\n");
    terminal.put_string(b"17\n");
    terminal.put_string(b"18\n");
    terminal.put_string(b"19\n");
    terminal.put_string(b"20\n");
    terminal.put_string(b"21\n");
    terminal.put_string(b"22\n");
    terminal.put_string(b"23\n");
    terminal.put_string(b"24\n");
    terminal.put_string(b"25\n");
    terminal.put_string(b"26");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
