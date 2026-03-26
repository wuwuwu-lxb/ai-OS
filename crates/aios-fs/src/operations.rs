//! File operations

use std::fs;
use std::path::Path;

pub struct FileOperations;

impl FileOperations {
    pub fn new() -> Self {
        Self
    }

    pub fn list_dir(&self, path: &str) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
        let mut names = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                names.push(entry.file_name().to_string_lossy().to_string());
            }
        }
        Ok(names)
    }

    pub fn read_file(&self, path: &str) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<(), String> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(path, content).map_err(|e| e.to_string())
    }
}
