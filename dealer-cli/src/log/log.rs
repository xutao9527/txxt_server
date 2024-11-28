use std::sync::OnceLock;
use chrono::{Datelike, Timelike, Utc};
use super::circular_buffer::CircularBuf;
static LOG_BUFFER: OnceLock<CircularBuf<String>> = OnceLock::new();

pub struct SLog;

impl SLog {
    pub fn init(capacity: usize) {
        LOG_BUFFER.get_or_init(|| CircularBuf::new(capacity));
    }

    pub fn log(message: String) {
        let now = Utc::now();
        let timestamp = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            now.timestamp_subsec_millis()
        );
        let log_message = format!("[{}] {}", timestamp, message);
        if let Some(buffer) = LOG_BUFFER.get() {
            buffer.push(log_message);
        } else {
            eprintln!("LOG_BUFFER is not initialized yet.");
        }
    }

    pub fn get(size: usize) -> Vec<String> {
        if let Some(buffer) = LOG_BUFFER.get() {
            buffer.get(size)
        } else {
            vec![]
        }
    }

    pub fn get_all() -> Vec<String> {
        if let Some(buffer) = LOG_BUFFER.get() {
            buffer.get_all()
        } else {
            vec![]
        }
    }
}
