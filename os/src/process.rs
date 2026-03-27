//! Process Management
//! Process Control Block and basic process management

pub const MAX_PROCESSES: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

#[repr(C)]
pub struct ProcessControlBlock {
    pub pid: u32,
    pub name: [u8; 32],
    pub state: ProcessState,
    pub priority: u8,
    pub time_slice: u32,
    pub stack_pointer: u64,
    pub instruction_pointer: u64,
    pub kernel_stack: u64,
    pub user_stack: u64,
}

impl ProcessControlBlock {
    pub fn new(pid: u32, name: &str) -> Self {
        let mut pcb = ProcessControlBlock {
            pid,
            name: [0; 32],
            state: ProcessState::Ready,
            priority: 5,
            time_slice: 10,
            stack_pointer: 0,
            instruction_pointer: 0,
            kernel_stack: 0,
            user_stack: 0,
        };

        let bytes = name.as_bytes();
        for (i, &byte) in bytes.iter().take(31).enumerate() {
            pcb.name[i] = byte;
        }

        pcb
    }
}

pub fn init() {
}

pub fn get_process_count() -> usize {
    2
}
