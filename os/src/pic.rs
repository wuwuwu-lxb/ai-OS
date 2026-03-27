//! Programmable Interrupt Controller (PIC) driver
//! Handles hardware interrupt routing

use core::ptr::{write_volatile, read_volatile};

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

const IRQ0_OFFSET: u8 = 0x20;
const IRQ8_OFFSET: u8 = 0x28;

pub fn init() {
    unsafe {
        write_volatile(PIC1_COMMAND as *mut u8, ICW1_INIT | ICW1_ICW4);
        write_volatile(PIC2_COMMAND as *mut u8, ICW1_INIT | ICW1_ICW4);

        write_volatile(PIC1_DATA as *mut u8, IRQ0_OFFSET);
        write_volatile(PIC2_DATA as *mut u8, IRQ8_OFFSET);

        write_volatile(PIC1_DATA as *mut u8, 0x04);
        write_volatile(PIC2_DATA as *mut u8, 0x02);

        write_volatile(PIC1_DATA as *mut u8, ICW4_8086);
        write_volatile(PIC2_DATA as *mut u8, ICW4_8086);

        write_volatile(PIC1_DATA as *mut u8, 0xFF);
        write_volatile(PIC2_DATA as *mut u8, 0xFF);
    }
}

pub fn enable_irq(irq: u8) {
    unsafe {
        if irq < 8 {
            let mask = read_volatile((PIC1_DATA) as *const u8);
            write_volatile((PIC1_DATA) as *mut u8, mask & !(1 << irq));
        } else {
            let mask = read_volatile((PIC2_DATA) as *const u8);
            write_volatile((PIC2_DATA) as *mut u8, mask & !(1 << (irq - 8)));
        }
    }
}

pub fn disable_irq(irq: u8) {
    unsafe {
        if irq < 8 {
            let mask = read_volatile((PIC1_DATA) as *const u8);
            write_volatile((PIC1_DATA) as *mut u8, mask | (1 << irq));
        } else {
            let mask = read_volatile((PIC2_DATA) as *const u8);
            write_volatile((PIC2_DATA) as *mut u8, mask | (1 << (irq - 8)));
        }
    }
}

pub fn end_of_interrupt(irq: u8) {
    unsafe {
        if irq >= 8 {
            write_volatile(PIC2_COMMAND as *mut u8, 0x20);
        }
        write_volatile(PIC1_COMMAND as *mut u8, 0x20);
    }
}
