//! Interrupt handling module
//! Manages all interrupt-related initialization and handlers

mod idt;
mod pic;
mod pit;
mod keyboard;

pub use idt::{init as idt_init, set_interrupt_gate};
pub use pic::{init as pic_init, enable_irq, end_of_interrupt};
pub use pit::{init as pit_init, tick as pit_tick, get_ticks};
pub use keyboard::{init as keyboard_init, handle_keypress, pop_key, has_keys};

#[no_mangle]
pub extern "C" fn pit_tick() {
    pit::tick();
}

#[no_mangle]
pub extern "C" fn keyboard_handle_keypress(scan_code: u64) {
    keyboard::handle_keypress(scan_code as u8);
}

pub fn init() {
    serial::write_string("[OK] Initializing PIC...\r\n");
    pic::init();
    serial::write_string("[OK] PIC initialized\r\n");

    serial::write_string("[OK] Initializing IDT...\r\n");
    idt::init();
    serial::write_string("[OK] IDT initialized\r\n");

    serial::write_string("[OK] Initializing PIT (100Hz)...\r\n");
    pit::init(100);
    serial::write_string("[OK] PIT initialized\r\n");

    serial::write_string("[OK] Enabling timer interrupt (IRQ0)...\r\n");
    pic::enable_irq(0);
    serial::write_string("[OK] Timer interrupt enabled\r\n");

    serial::write_string("[OK] Initializing keyboard...\r\n");
    keyboard::init();
    serial::write_string("[OK] Keyboard initialized\r\n");

    serial::write_string("[OK] Enabling keyboard interrupt (IRQ1)...\r\n");
    pic::enable_irq(1);
    serial::write_string("[OK] Keyboard interrupt enabled\r\n");

    serial::write_string("[OK] All interrupts enabled!\r\n");
    serial::write_string("\r\n");
}
