//! CLI Commands

/// Available CLI commands
#[derive(Debug)]
pub enum Command {
    Exit,
    Clear,
    Help,
    History,
    Unknown(String),
}

impl Command {
    /// Parse input into a command
    pub fn parse(input: &str) -> Self {
        match input.trim() {
            "/exit" | "/quit" => Command::Exit,
            "/clear" | "/cls" => Command::Clear,
            "/help" | "/?" => Command::Help,
            "/history" | "/hist" => Command::History,
            other if other.starts_with('/') => Command::Unknown(other.to_string()),
            _ => Command::Unknown(input.to_string()),
        }
    }

    /// Check if input is a command
    pub fn is_command(input: &str) -> bool {
        input.trim().starts_with('/')
    }
}
