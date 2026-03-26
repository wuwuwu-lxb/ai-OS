//! Policy and Permission engine for AI-OS

mod permissions;
mod audit;

pub use permissions::PermissionChecker;
pub use audit::AuditLog;

/// Permission action types
#[derive(Debug, Clone)]
pub enum Action {
    Read,
    Write,
    Execute,
    Delete,
    NetworkAccess,
    ProcessControl,
}

/// Resource types
#[derive(Debug, Clone)]
pub enum Resource {
    File(String),
    Directory(String),
    Process(u32),
    NetworkEndpoint(String),
}
