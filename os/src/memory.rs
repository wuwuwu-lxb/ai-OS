//! Physical Memory Management
//! Handles physical memory detection and allocation

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub base: u64,
    pub length: u64,
    pub region_type: MemoryRegionType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryRegionType {
    Available,
    Reserved,
    ACPIReclaimable,
    NVS,
    BadRAM,
}

impl MemoryRegionType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => MemoryRegionType::Available,
            2 => MemoryRegionType::Reserved,
            3 => MemoryRegionType::ACPIReclaimable,
            4 => MemoryRegionType::NVS,
            5 => MemoryRegionType::BadRAM,
            _ => MemoryRegionType::Reserved,
        }
    }
}

pub fn init() {
}

pub fn get_total_memory() -> u64 {
    256 * 1024 * 1024
}
