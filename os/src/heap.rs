//! Heap Memory Allocator
//! Implements a simple bump allocator for kernel heap

pub const HEAP_START: u64 = 0xFFFF_8000_0100_0000;
pub const HEAP_SIZE: usize = 16 * 1024 * 1024;
pub const HEAP_END: u64 = HEAP_START + HEAP_SIZE as u64;

static mut HEAP_NEXT: u64 = HEAP_START;
static mut HEAP_ALLOCATIONS: usize = 0;

pub fn init() {
    unsafe {
        HEAP_NEXT = HEAP_START;
        HEAP_ALLOCATIONS = 0;
    }
}

pub fn allocate(size: usize, align: usize) -> Option<u64> {
    unsafe {
        let mut ptr = HEAP_NEXT;

        let align_mask = !(align as u64 - 1);
        ptr = (ptr + align as u64 - 1) & align_mask;

        if ptr + size as u64 > HEAP_END {
            return None;
        }

        let result = ptr;
        HEAP_NEXT = ptr + size as u64;
        HEAP_ALLOCATIONS += 1;

        Some(result)
    }
}

pub fn get_used_memory() -> u64 {
    unsafe {
        HEAP_NEXT - HEAP_START
    }
}

pub fn get_allocation_count() -> usize {
    unsafe {
        HEAP_ALLOCATIONS
    }
}
