//! Process monitoring

pub struct ProcessMonitor;

impl ProcessMonitor {
    pub fn new() -> Self {
        Self
    }

    /// List running processes (simplified)
    pub fn list_processes(&self) -> Vec<(u32, String)> {
        vec![]
    }
}
