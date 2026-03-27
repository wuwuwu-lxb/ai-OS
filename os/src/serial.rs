//! Serial Port Driver for AI-OS Kernel

use core::ptr::{write_volatile, read_volatile};

/// COM1 I/O port base address
const COM1: u16 = 0x3F8;

/// Serial port registers (offset from base port)
const REG_DATA: u16 = 0;
const REG_INTERRUPT_ENABLE: u16 = 1;
const REG_LINE_CONTROL: u16 = 3;
const REG_MODEM_CONTROL: u16 = 4;
const REG_LINE_STATUS: u16 = 5;

/// Initialize serial port (COM1)
pub fn init() {
    unsafe {
        write_volatile(COM1 as *mut u8, 0x00);
        write_volatile((COM1 + REG_LINE_CONTROL) as *mut u8, 0x80);
        write_volatile((COM1 + REG_DATA) as *mut u8, 0x0C);
        write_volatile((COM1 + REG_INTERRUPT_ENABLE) as *mut u8, 0x00);
        write_volatile((COM1 + REG_LINE_CONTROL) as *mut u8, 0x03);
        write_volatile((COM1 + REG_INTERRUPT_ENABLE) as *mut u8, 0xC7);
        write_volatile((COM1 + REG_MODEM_CONTROL) as *mut u8, 0x0B);
        write_volatile((COM1 + REG_MODEM_CONTROL) as *mut u8, 0x1E);
        write_volatile((COM1 + REG_DATA) as *mut u8, 0xAE);
        if read_volatile((COM1 + REG_DATA) as *const u8) != 0xAE {
            return;
        }
        write_volatile((COM1 + REG_MODEM_CONTROL) as *mut u8, 0x0F);
    }
}

/// Write a character to serial port
pub fn write_char(c: u8) {
    unsafe {
        while (read_volatile((COM1 + REG_LINE_STATUS) as *const u8) & 0x20) == 0 {}
        write_volatile((COM1 + REG_DATA) as *mut u8, c);
    }
}

/// Write a string to serial port
pub fn write_string(s: &str) {
    for byte in s.bytes() {
        write_char(byte);
    }
}

pub fn write_u64(n: u64) {
    let mut buffer = [0u8; 20];
    let mut i = 0;

    if n == 0 {
        write_char(b'0');
        return;
    }

    let mut num = n;
    while num > 0 {
        buffer[19 - i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    for k in (20 - i)..20 {
        write_char(buffer[k]);
    }
}

pub fn write_u32(n: u32) {
    write_u64(n as u64);
}
