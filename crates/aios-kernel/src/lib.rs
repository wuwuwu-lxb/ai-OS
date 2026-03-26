//! Linux Kernel Interface Layer

mod syscall;
mod procfs;

pub use syscall::SyscallBridge;
pub use procfs::ProcfsMonitor;
