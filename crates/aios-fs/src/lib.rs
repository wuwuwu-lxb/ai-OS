//! Filesystem Abstraction for AI-OS

mod ai_path;
mod operations;

pub use ai_path::AIPathResolver;
pub use operations::FileOperations;

/// Filesystem operations that AI can perform
pub trait FileSystem {
    fn list_dir(&self, path: &str) -> Result<Vec<String>, String>;
    fn read_file(&self, path: &str) -> Result<String, String>;
    fn write_file(&self, path: &str, content: &str) -> Result<(), String>;
    fn delete_file(&self, path: &str) -> Result<(), String>;
}
