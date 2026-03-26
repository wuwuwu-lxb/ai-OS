//! Audit logging for security events

use chrono::{DateTime, Utc};

pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

#[derive(Debug)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub resource: String,
    pub result: bool,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, action: &str, resource: &str, result: bool) {
        self.entries.push(AuditEntry {
            timestamp: Utc::now(),
            action: action.to_string(),
            resource: resource.to_string(),
            result,
        });
    }
}
