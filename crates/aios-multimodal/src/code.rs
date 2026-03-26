//! Code processor

pub struct CodeProcessor;

impl CodeProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_language(&self, code: &str) -> Option<String> {
        // Simple language detection based on keywords
        if code.contains("fn ") && code.contains("let ") {
            Some("rust".to_string())
        } else if code.contains("func ") && code.contains("package ") {
            Some("go".to_string())
        } else if code.contains("def ") && code.contains(":") {
            Some("python".to_string())
        } else if code.contains("function") || code.contains("const ") {
            Some("javascript".to_string())
        } else {
            None
        }
    }
}
