//! Process Management for AI-OS

mod spawn;
mod monitor;

pub use spawn::ProcessSpawner;
pub use monitor::ProcessMonitor;

/// Process information
#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub status: String,
}
