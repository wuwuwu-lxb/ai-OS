//! Capability Registry
//! System for registering and discovering AI capabilities

pub const MAX_CAPABILITIES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapabilityType {
    Tool,
    Skill,
    Plugin,
    API,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Capability {
    pub id: u32,
    pub name: [u8; 32],
    pub description: [u8; 128],
    pub cap_type: CapabilityType,
    pub version: u16,
    pub execute_fn: u64,
    pub enabled: bool,
}

impl Capability {
    pub fn new(id: u32, name: &str, description: &str, cap_type: CapabilityType) -> Self {
        let mut cap = Capability {
            id,
            name: [0; 32],
            description: [0; 128],
            cap_type,
            version: 1,
            execute_fn: 0,
            enabled: true,
        };

        let name_bytes = name.as_bytes();
        for (i, &byte) in name_bytes.iter().take(31).enumerate() {
            cap.name[i] = byte;
        }

        let desc_bytes = description.as_bytes();
        for (i, &byte) in desc_bytes.iter().take(127).enumerate() {
            cap.description[i] = byte;
        }

        cap
    }
}

static mut CAPABILITIES: [Option<Capability>; MAX_CAPABILITIES] = unsafe {
    [None; MAX_CAPABILITIES]
};
static mut NEXT_CAPABILITY_ID: u32 = 1;
static mut CAPABILITY_COUNT: usize = 0;

pub fn init() {
}

pub fn register(
    name: &str,
    description: &str,
    cap_type: CapabilityType
) -> Option<u32> {
    unsafe {
        if CAPABILITY_COUNT >= MAX_CAPABILITIES {
            return None;
        }

        let id = NEXT_CAPABILITY_ID;
        NEXT_CAPABILITY_ID += 1;

        let cap = Capability::new(id, name, description, cap_type);

        for i in 0..MAX_CAPABILITIES {
            if CAPABILITIES[i].is_none() {
                CAPABILITIES[i] = Some(cap);
                CAPABILITY_COUNT += 1;
                return Some(id);
            }
        }

        None
    }
}

pub fn unregister(id: u32) -> bool {
    unsafe {
        for i in 0..MAX_CAPABILITIES {
            if let Some(_) = CAPABILITIES[i] {
                if CAPABILITIES[i].unwrap().id == id {
                    CAPABILITIES[i] = None;
                    CAPABILITY_COUNT -= 1;
                    return true;
                }
            }
        }
    }
    false
}

pub fn enable(id: u32) -> bool {
    unsafe {
        for i in 0..MAX_CAPABILITIES {
            if let Some(ref mut cap) = CAPABILITIES[i] {
                if cap.id == id {
                    cap.enabled = true;
                    return true;
                }
            }
        }
    }
    false
}

pub fn disable(id: u32) -> bool {
    unsafe {
        for i in 0..MAX_CAPABILITIES {
            if let Some(ref mut cap) = CAPABILITIES[i] {
                if cap.id == id {
                    cap.enabled = false;
                    return true;
                }
            }
        }
    }
    false
}

pub fn get_capability_count() -> usize {
    unsafe { CAPABILITY_COUNT }
}

pub fn list_capabilities() {
}
