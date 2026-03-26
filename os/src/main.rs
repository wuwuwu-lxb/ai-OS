//! AI-OS Kernel - Minimal x86_64 OS
//!
//! This is a minimal bare-metal kernel that can boot on x86_64.

#![no_std]
#![no_main]

mod vga;
mod gdt;

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize VGA display
    vga::init();
    vga::clear_screen();
    vga::write_string("AI-OS Kernel v0.1.0\n");
    vga::write_string("==================\n\n");

    // Initialize GDT
    gdt::init();
    vga::write_string("[OK] GDT initialized\n");
    vga::write_string("[OK] Kernel loaded!\n");
    vga::write_string("\nSystem halted.\n");

    // Halt using inline assembly
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
