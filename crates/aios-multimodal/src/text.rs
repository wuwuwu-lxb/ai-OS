//! Text processor

pub struct TextProcessor;

impl TextProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, input: &str) -> String {
        input.trim().to_string()
    }
}
