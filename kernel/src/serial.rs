use crate::io::{in8, out8};
use crate::serial::SerialError::SerialPortInitializationFailed;

const COM1_PORT: u16 = 0x3F8;
const OFFSET_DATA_REGISTER: u8 = 0;
const OFFSET_INTERRUPT_ENABLE_REGISTER: u8 = 1;
const OFFSET_INTERRUPT_IDENTIFICATION_AND_FIFO_CONTROL_REGISTER: u8 = 2;
const OFFSET_LINE_CONTROL_REGISTER: u8 = 3;
const OFFSET_MODEM_CONTROL_REGISTER: u8 = 4;
const OFFSET_LINE_STATUS_REGISTER: u8 = 5;

#[derive(Debug)]
pub enum SerialError {
    SerialPortInitializationFailed,
}

pub fn initialize_serial_port() -> Result<(), SerialError> {
    // Safety:
    // These instructions read and write data on valid COM ports
    unsafe {
        out8(COM1_PORT + OFFSET_INTERRUPT_ENABLE_REGISTER as u16, 0x00);
        out8(COM1_PORT + OFFSET_LINE_CONTROL_REGISTER as u16, 0x80);
        out8(COM1_PORT + OFFSET_DATA_REGISTER as u16, 0x03);
        out8(COM1_PORT + OFFSET_INTERRUPT_ENABLE_REGISTER as u16, 0x00);
        out8(COM1_PORT + OFFSET_LINE_CONTROL_REGISTER as u16, 0x03);
        out8(
            COM1_PORT + OFFSET_INTERRUPT_IDENTIFICATION_AND_FIFO_CONTROL_REGISTER as u16,
            0xC7,
        );
        out8(COM1_PORT + OFFSET_MODEM_CONTROL_REGISTER as u16, 0x0B);

        // Testing the serial chip
        out8(COM1_PORT + OFFSET_MODEM_CONTROL_REGISTER as u16, 0x1E); // Enable loopback mode
        out8(COM1_PORT + OFFSET_DATA_REGISTER as u16, 0xAE);
        if in8(COM1_PORT + OFFSET_DATA_REGISTER as u16) != 0xAE {
            return Err(SerialPortInitializationFailed);
        }

        out8(COM1_PORT + OFFSET_MODEM_CONTROL_REGISTER as u16, 0x0F); // Enable normal mode
    }

    Ok(())
}

pub fn write(byte: u8) {
    while !is_ready_to_send_data() {}
    // Safety: the value is sent on a valid port address, which is the COM port
    // data register address
    unsafe {
        out8(COM1_PORT, byte);
    }
}

pub fn write_bytes(bytes: &[u8]) {
    for &byte in bytes {
        write(byte);
    }
}

fn is_ready_to_send_data() -> bool {
    const THRE_BIT: u8 = 0x20;

    // Safety: the value is sent on a valid port address, which is the COM port
    // line status register address
    return unsafe { in8(COM1_PORT + OFFSET_LINE_STATUS_REGISTER as u16) } & THRE_BIT != 0;
}
