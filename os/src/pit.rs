//! Programmable Interval Timer (PIT) driver
//! Provides periodic interrupts for system timing

use core::ptr::{write_volatile, read_volatile};

const PIT_COMMAND: u16 = 0x43;
const PIT_CHANNEL0: u16 = 0x40;

const PIT_MODE_RATE_GENERATOR: u8 = 0x36;

pub static mut TICK_COUNT: u64 = 0;

pub fn init(frequency: u32) {
    let divisor = 1193182 / frequency;

    unsafe {
        write_volatile((PIT_COMMAND) as *mut u8, PIT_MODE_RATE_GENERATOR);

        write_volatile((PIT_CHANNEL0) as *mut u8, (divisor & 0xFF) as u8);
        write_volatile((PIT_CHANNEL0) as *mut u8, ((divisor >> 8) & 0xFF) as u8);

        TICK_COUNT = 0;
    }
}

pub fn tick() {
    unsafe {
        TICK_COUNT += 1;
    }
}

pub fn get_ticks() -> u64 {
    unsafe { TICK_COUNT }
}

pub fn wait_ms(milliseconds: u32) {
    let start = get_ticks();
    let target_ms = milliseconds as u64 * 100;
    while get_ticks() - start < target_ms {}
}
