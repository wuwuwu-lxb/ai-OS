//! Procfs monitor

pub struct ProcfsMonitor;

impl ProcfsMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn get_load_average(&self) -> Result<(f64, f64, f64), String> {
        let content = std::fs::read_to_string("/proc/loadavg")
            .map_err(|e| e.to_string())?;

        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 {
            Ok((
                parts[0].parse().unwrap_or(0.0),
                parts[1].parse().unwrap_or(0.0),
                parts[2].parse().unwrap_or(0.0),
            ))
        } else {
            Err("Invalid loadavg format".to_string())
        }
    }
}
