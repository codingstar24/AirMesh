//! Output formatting for CLI: human tables vs machine-readable JSON.

use serde::Serialize;

/// Formats and prints data according to `--json` flag.
pub fn print_output<T: Serialize + std::fmt::Display>(data: &T, as_json: bool) {
    if as_json {
        let json_str = serde_json::to_string_pretty(data)
            .unwrap_or_else(|_| "{}".to_string());
        println!("{json_str}");
    } else {
        println!("{data}");
    }
}

/// Helper to format raw byte count into human-readable string.
pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let i = (bytes as f64).log(1024.0).floor() as usize;
    let i = i.min(UNITS.len() - 1);
    let val = (bytes as f64) / 1024.0_f64.powi(i as i32);
    format!("{:.2} {}", val, UNITS[i])
}
