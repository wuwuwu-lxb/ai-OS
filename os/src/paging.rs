//! Virtual Memory and Paging
//! Implements x86_64 paging

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_BITS: usize = 12;
pub const ENTRY_COUNT: usize = 512;

pub const PTE_PRESENT: u64 = 1 << 0;
pub const PTE_WRITABLE: u64 = 1 << 1;
pub const PTE_USER: u64 = 1 << 2;

#[repr(align(4096))]
pub struct PageTable {
    pub entries: [u64; ENTRY_COUNT],
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            entries: [0; ENTRY_COUNT],
        }
    }

    pub fn set_entry(&mut self, index: usize, entry: u64) {
        self.entries[index] = entry;
    }

    pub fn get_entry(&self, index: usize) -> u64 {
        self.entries[index]
    }

    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            *entry = 0;
        }
    }
}

pub fn init() {
}

pub fn allocate_page() -> Option<u64> {
    None
}

pub fn get_heap_start() -> u64 {
    0xFFFF_8000_0000_0000
}
