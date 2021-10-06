use crate::io;
use compiler_builtins::mem::memcpy;

const VGA_BUFFER_POINTER: *mut u8 = 0xb8000 as *mut u8;
const VGA_BUFFER_SIZE: u16 = 4000;
const ROW_COUNT: u8 = 25;
const COL_COUNT: u8 = 80;
const VALUE_SIZE: u8 = 2;

pub(crate) struct Terminal {
    current_row: u8,
    current_col: u8,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            current_row: 0,
            current_col: 0,
        }
    }

    pub fn put_string(&mut self, string: &[u8]) {
        for &character in string {
            self.put_char(character);
        }
        self.set_cursor_position(self.current_row, self.current_col);
    }

    pub fn put_char(&mut self, c: u8) {
        if c == b'\n' {
            if self.current_row < ROW_COUNT - 1 {
                self.current_row += 1;
            } else {
                self.scroll_y();
            }

            self.current_col = 0;
            return;
        }

        /// Safety: This is safe because VGA_BUFFER_POINTER points toward the VGA character buffer
        unsafe {
            *VGA_BUFFER_POINTER.offset(
                ((self.current_col as isize + self.current_row as isize * COL_COUNT as isize)
                    * VALUE_SIZE as isize),
            ) = c;
        }
        self.compute_current_position();
    }

    fn compute_current_position(&mut self) {
        if self.current_col == COL_COUNT - 1 {
            if self.current_row != ROW_COUNT - 1 {
                self.current_row = (self.current_row + 1) % ROW_COUNT;
            } else {
                self.scroll_y();
            }
            self.current_col = 0;
        } else {
            self.current_col = (self.current_col + 1) % COL_COUNT;
        }
    }

    fn scroll_y(&mut self) {
        // Safety:
        // The starting pointer is in bound because it's the VGA character buffer pointer
        // The end pointer isn't past the VGA character buffer pointer
        // The offset doesn't overflow isize
        // The offset doesn't rely on wrapping around the address space
        let src = unsafe { VGA_BUFFER_POINTER.offset(COL_COUNT as isize * VALUE_SIZE as isize) };
        let dest = VGA_BUFFER_POINTER;
        let size = (VGA_BUFFER_SIZE as usize - COL_COUNT as usize) * VALUE_SIZE as usize;

        // Safety:
        // dest and src are valid pointer to the same object being the VGA character buffer
        // This copy will not go  past the VGA character buffer
        unsafe { memcpy(dest, src, size) };

        self.clear_line(ROW_COUNT - 1);
    }

    pub fn put_char_at(&mut self, c: u8, row: u8, col: u8) {
        self.current_row = row;
        self.current_col = col;
        unsafe {
            *VGA_BUFFER_POINTER.offset(
                (self.current_col as isize + self.current_row as isize * COL_COUNT as isize)
                    * VALUE_SIZE as isize,
            ) = c;
        }
    }

    pub fn clear(&mut self) {
        for row in 0..ROW_COUNT {
            self.clear_line(row)
        }
        self.current_col = 0;
        self.current_row = 0;
    }

    fn clear_line(&mut self, row: u8) {
        for col in 0..COL_COUNT {
            self.put_char_at(b' ', row, col);
        }

        self.current_row = row;
        self.current_col = 0;
    }

    pub fn set_cursor_position(&mut self, row: u8, col: u8) {
        let cursor_position: u16 = (row * COL_COUNT + col) as u16;
        let address_register = 0x3D4;
        let data_register = 0x3D5;
        unsafe {
            io::out8(address_register, 0x0F);
            io::out8(data_register, (cursor_position & 0xFF) as u8);
            io::out8(address_register, 0x0E);
            io::out8(data_register, ((cursor_position >> 8) & 0xFF) as u8);
        }
    }
}
