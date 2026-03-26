//! Syscall bridge - safe wrappers for Linux syscalls

pub struct SyscallBridge;

impl SyscallBridge {
    pub fn new() -> Self {
        Self
    }

    /// Get system uptime
    pub fn uptime(&self) -> Result<u64, String> {
        // Read from /proc/uptime
        std::fs::read_to_string("/proc/uptime")
            .map_err(|e| e.to_string())
            .and_then(|s| {
                s.split_whitespace()
                    .next()
                    .ok_or_else(|| "Invalid uptime format".to_string())
                    .and_then(|v| v.parse::<u64>().map_err(|e| e.to_string()))
            })
    }
}
