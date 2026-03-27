//! AI-OS Kernel - Minimal x86_64 OS
//!
//! This is a minimal bare-metal kernel that can boot on x86_64.

#![no_std]
#![no_main]

mod vga;
mod gdt;
mod serial;
mod pic;
mod pit;
mod keyboard;
mod idt;
mod memory;
mod paging;
mod heap;
mod process;
mod scheduler;
mod context_switch;

fn write_u64(n: u64) {
    let mut buffer = [0u8; 20];
    let mut i = 0;

    if n == 0 {
        serial::write_char(b'0');
        return;
    }

    let mut num = n;
    while num > 0 {
        buffer[19 - i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    for k in (20 - i)..20 {
        serial::write_char(buffer[k]);
    }
}

fn write_u32(n: u32) {
    write_u64(n as u64);
}

extern "C" fn idle_task() {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial::init();
    serial::write_string("\r\n==========================================\r\n");
    serial::write_string("  AI-OS Kernel v0.4.0 - Process System\r\n");
    serial::write_string("==========================================\r\n\r\n");

    vga::init();
    vga::clear_screen();
    vga::write_string("AI-OS Kernel v0.4.0\r\n");
    vga::write_string("==================\r\n\r\n");

    serial::write_string("[OK] Serial port initialized\r\n");
    vga::write_string("[OK] Serial initialized\r\n");

    serial::write_string("[OK] Initializing GDT...\r\n");
    gdt::init();
    serial::write_string("[OK] GDT initialized\r\n");
    vga::write_string("[OK] GDT initialized\r\n");

    serial::write_string("\r\n[OK] Initializing interrupt system...\r\n");
    vga::write_string("[OK] Initializing interrupts...\r\n");

    serial::write_string("[OK] Initializing PIC...\r\n");
    pic::init();

    serial::write_string("[OK] Initializing IDT...\r\n");
    idt::init();

    serial::write_string("[OK] Initializing PIT (100Hz)...\r\n");
    pit::init(100);

    serial::write_string("[OK] Enabling timer interrupt (IRQ0)...\r\n");
    pic::enable_irq(0);

    serial::write_string("[OK] Initializing keyboard...\r\n");
    keyboard::init();

    serial::write_string("[OK] Enabling keyboard interrupt (IRQ1)...\r\n");
    pic::enable_irq(1);

    serial::write_string("[OK] All interrupts enabled!\r\n");
    serial::write_string("\r\n");

    serial::write_string("\r\n[OK] Initializing memory system...\r\n");
    vga::write_string("[OK] Initializing memory...\r\n");

    serial::write_string("[OK] Detecting physical memory...\r\n");
    memory::init();

    serial::write_string("[OK] Setting up paging...\r\n");
    paging::init();

    serial::write_string("[OK] Initializing heap allocator...\r\n");
    heap::init();

    serial::write_string("\r\n[OK] Initializing process system...\r\n");
    vga::write_string("[OK] Initializing processes...\r\n");

    serial::write_string("[OK] Initializing context switch...\r\n");
    context_switch::init();

    serial::write_string("[OK] Initializing scheduler...\r\n");
    scheduler::init();

    serial::write_string("[OK] Creating idle process...\r\n");
    process::init();

    serial::write_string("[INFO] Process structures ready\r\n");

    serial::write_string("\r\n[INFO] Total system memory: ");
    write_u64(memory::get_total_memory() / (1024 * 1024));
    serial::write_string(" MB\r\n");

    serial::write_string("[OK] Heap used: ");
    write_u64(heap::get_used_memory() / 1024);
    serial::write_string(" KB\r\n");

    serial::write_string("[OK] Active processes: ");
    write_u32(process::get_process_count() as u32);
    serial::write_string("\r\n");

    serial::write_string("\r\n[OK] AI-OS Kernel v0.4.0 is running!\r\n");
    serial::write_string("[OK] Process Management: Ready\r\n");
    serial::write_string("[OK] Scheduler: Active\r\n");
    serial::write_string("[OK] Timer: 100 Hz\r\n");
    serial::write_string("\r\n");

    vga::write_string("[OK] Kernel running!\r\n");
    vga::write_string("Process Management Ready\r\n\r\n");

    let mut tick_counter: u64 = 0;
    let mut seconds: u64 = 0;

    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)); }

        let current_ticks = pit::get_ticks();
        if tick_counter < current_ticks {
            tick_counter = current_ticks;
            if tick_counter % 100 == 0 {
                seconds += 1;
                scheduler::on_tick();
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial::write_string("\r\n[PANIC] Kernel panic!\r\n");
    loop {}
}
