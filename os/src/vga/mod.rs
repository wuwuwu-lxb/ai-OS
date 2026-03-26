//! VGA Text Mode Driver

use core::ptr::{write_volatile, read_volatile};

/// VGA memory base address
const VGA_ADDRESS: *mut u8 = 0xb8000 as *mut u8;

/// VGA width
const VGA_WIDTH: usize = 80;

/// VGA height
const VGA_HEIGHT: usize = 25;

/// Current cursor position
static mut CURSOR: (usize, usize) = (0, 0);

/// Initialize VGA
pub fn init() {
    unsafe { CURSOR = (0, 0); }
}

/// Clear the screen
pub fn clear_screen() {
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            unsafe {
                let offset = (y * VGA_WIDTH + x) * 2;
                write_volatile(VGA_ADDRESS.add(offset), b' ');
                write_volatile(VGA_ADDRESS.add(offset + 1), 0x07);
            }
        }
    }
}

/// Write a character
pub fn write_char(c: char) {
    let (mut x, mut y) = get_cursor();

    match c {
        '\n' => {
            x = 0;
            y += 1;
        }
        '\r' => {
            x = 0;
        }
        _ => {
            unsafe {
                let offset = (y * VGA_WIDTH + x) * 2;
                write_volatile(VGA_ADDRESS.add(offset), c as u8);
                write_volatile(VGA_ADDRESS.add(offset + 1), 0x07);
            }
            x += 1;
            if x >= VGA_WIDTH {
                x = 0;
                y += 1;
            }
        }
    }

    if y >= VGA_HEIGHT {
        scroll();
        y = VGA_HEIGHT - 1;
    }

    set_cursor(x, y);
}

/// Scroll the screen up by one line
fn scroll() {
    for dy in 1..VGA_HEIGHT {
        for dx in 0..VGA_WIDTH {
            unsafe {
                let src_offset = (dy * VGA_WIDTH + dx) * 2;
                let dst_offset = ((dy - 1) * VGA_WIDTH + dx) * 2;
                let byte = read_volatile(VGA_ADDRESS.add(src_offset));
                let color = read_volatile(VGA_ADDRESS.add(src_offset + 1));
                write_volatile(VGA_ADDRESS.add(dst_offset), byte);
                write_volatile(VGA_ADDRESS.add(dst_offset + 1), color);
            }
        }
    }
    // Clear last line
    for dx in 0..VGA_WIDTH {
        unsafe {
            let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + dx) * 2;
            write_volatile(VGA_ADDRESS.add(offset), b' ');
            write_volatile(VGA_ADDRESS.add(offset + 1), 0x07);
        }
    }
}

/// Get current cursor position
fn get_cursor() -> (usize, usize) {
    unsafe { CURSOR }
}

/// Set cursor position
fn set_cursor(x: usize, y: usize) {
    unsafe { CURSOR = (x, y); }
}

/// Write a string
pub fn write_string(s: &str) {
    for c in s.chars() {
        write_char(c);
    }
}
