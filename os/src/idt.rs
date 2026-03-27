//! Interrupt Descriptor Table (IDT) implementation
//! Handles hardware and software interrupts

use core::mem::size_of;

pub const IDT_ENTRIES: usize = 256;

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    fn new(offset: u64, selector: u16, flags: u8) -> Self {
        IdtEntry {
            offset_low: offset as u16,
            selector,
            ist: 0,
            flags,
            offset_mid: (offset >> 16) as u16,
            offset_high: (offset >> 32) as u32,
            reserved: 0,
        }
    }
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}

static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry { offset_low: 0, selector: 0, ist: 0, flags: 0, offset_mid: 0, offset_high: 0, reserved: 0 }; IDT_ENTRIES];

pub const IDT_FLAGS_PRESENT: u8    = 0b10000000;
pub const IDT_FLAGS_RING0: u8      = 0b00000000;
pub const IDT_FLAGS_RING3: u8       = 0b01100000;
pub const IDT_FLAGS_INTERRUPT_GATE: u8 = 0b00001110;
pub const IDT_FLAGS_TRAP_GATE: u8    = 0b00001111;

extern "C" {
    fn load_idt(idt_ptr: *const IdtPtr);
    fn enable_interrupts();
}

pub fn init() {
    unsafe {
        for i in 0..IDT_ENTRIES {
            IDT[i] = IdtEntry {
                offset_low: 0,
                selector: 0,
                ist: 0,
                flags: 0,
                offset_mid: 0,
                offset_high: 0,
                reserved: 0,
            };
        }

        let mut idt_ptr = IdtPtr {
            limit: ((size_of::<IdtEntry>() * IDT_ENTRIES) - 1) as u16,
            base: 0,
        };

        idt_ptr.base = &IDT as *const _ as u64;

        load_idt(&idt_ptr);

        enable_interrupts();
    }
}

pub fn set_interrupt_gate(vector: u8, handler: u64, selector: u16) {
    let flags = IDT_FLAGS_PRESENT | IDT_FLAGS_RING0 | IDT_FLAGS_INTERRUPT_GATE;
    unsafe {
        IDT[vector as usize] = IdtEntry::new(handler, selector, flags);
    }
}
