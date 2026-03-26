//! Process spawning

use std::process::Command;

pub struct ProcessSpawner;

impl ProcessSpawner {
    pub fn new() -> Self {
        Self
    }

    /// Spawn a new process
    pub fn spawn(&self, program: &str, args: &[&str]) -> Result<u32, String> {
        let output = Command::new(program)
            .args(args)
            .spawn()
            .map_err(|e| e.to_string())?;

        Ok(output.id())
    }
}
