//! Global Descriptor Table (GDT) implementation
//! This module sets up the GDT for x86_64 protected mode

use core::mem::size_of;

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags: u8,
    base_high: u8,
}

impl GdtEntry {
    fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        GdtEntry {
            limit_low: limit as u16,
            base_low: base as u16,
            base_mid: (base >> 16) as u8,
            access: access,
            flags: flags,
            base_high: (base >> 24) as u8,
        }
    }
}

const GDT_ENTRIES: usize = 6;

static mut GDT: [GdtEntry; GDT_ENTRIES] = [
    GdtEntry { limit_low: 0, base_low: 0, base_mid: 0, access: 0, flags: 0, base_high: 0 };
    GDT_ENTRIES
];

#[repr(packed)]
struct Pseudodescriptor {
    limit: u16,
    base: u64,
}

const GDT_ACCESS_PRESENT: u8       = 0b10000000;
const GDT_ACCESS_DPL_RING0: u8      = 0b00000000;
const GDT_ACCESS_DPL_RING3: u8      = 0b01100000;
const GDT_ACCESS_SYSTEM: u8        = 0b00000000;
const GDT_ACCESS_EXECUTABLE: u8     = 0b00010000;
const GDT_ACCESS_CONFORMING: u8     = 0b00001000;
const GDT_ACCESS_READ_WRITE: u8     = 0b00000010;
const GDT_ACCESS_ACCESSED: u8      = 0b00000001;

const GDT_FLAGS_Granularity: u8    = 0b10000000;
const GDT_FLAGS_Size: u8           = 0b01000000;
const GDT_FLAGS_LongMode: u8        = 0b00100000;

pub fn init() {
    unsafe {
        let kernel_code_base: u32 = 0x00000000;
        let kernel_data_base: u32 = 0x00000000;
        let user_code_base: u32 = 0x00000000;
        let user_data_base: u32 = 0x00000000;

        let kernel_limit: u32 = 0xFFFFF;
        let user_limit: u32 = 0xFFFFF;

        let kernel_code_access = GDT_ACCESS_PRESENT
            | GDT_ACCESS_DPL_RING0
            | GDT_ACCESS_EXECUTABLE
            | GDT_ACCESS_READ_WRITE;

        let kernel_data_access = GDT_ACCESS_PRESENT
            | GDT_ACCESS_DPL_RING0
            | GDT_ACCESS_READ_WRITE;

        let user_code_access = GDT_ACCESS_PRESENT
            | GDT_ACCESS_DPL_RING3
            | GDT_ACCESS_EXECUTABLE
            | GDT_ACCESS_READ_WRITE;

        let user_data_access = GDT_ACCESS_PRESENT
            | GDT_ACCESS_DPL_RING3
            | GDT_ACCESS_READ_WRITE;

        let kernel_flags = GDT_FLAGS_Granularity | GDT_FLAGS_Size;
        let user_flags = GDT_FLAGS_Granularity | GDT_FLAGS_Size;

        GDT[0] = GdtEntry::new(0, 0, 0, 0);

        GDT[1] = GdtEntry::new(kernel_code_base, kernel_limit, kernel_code_access, kernel_flags);

        GDT[2] = GdtEntry::new(kernel_data_base, kernel_limit, kernel_data_access, kernel_flags);

        GDT[3] = GdtEntry::new(user_code_base, user_limit, user_code_access, user_flags);

        GDT[4] = GdtEntry::new(user_data_base, user_limit, user_data_access, user_flags);

        let mut gdtr = Pseudodescriptor {
            limit: ((size_of::<GdtEntry>() * GDT_ENTRIES) - 1) as u16,
            base: 0,
        };

        gdtr.base = &GDT as *const _ as u64;

        load_gdt(&gdtr);

        reload_segments();
    }
}

extern "C" {
    fn load_gdt(gdtr: *const Pseudodescriptor);
    fn reload_segments();
}
