use super::circular_buffer::CircularBuf;
use chrono::{Datelike, Timelike, Utc};
use std::sync::OnceLock;
static LOG_BUFFER: OnceLock<CircularBuf<LogMsg>> = OnceLock::new();

#[derive(Debug,Clone)]
pub enum LogType {
    INFO,
    ERROR,
}

#[derive(Debug,Clone)]
pub struct LogMsg {
    pub log_type: LogType,
    pub log_content: String,
}
pub struct SLog;

impl SLog {
    pub fn init(capacity: usize) {
        LOG_BUFFER.get_or_init(|| CircularBuf::<LogMsg>::new(capacity));
    }

    pub fn info(message: String) {
        let log_message = format!("[{}] {}", SLog::timestamp(), message);
        if let Some(buffer) = LOG_BUFFER.get() {
            buffer.push(LogMsg{
                log_type:LogType::INFO,
                log_content:log_message
            });
        } else {
            eprintln!("LOG_BUFFER is not initialized yet.");
        }
    }

    pub fn err(message: String) {
        let log_message = format!("[{}] {}", SLog::timestamp(), message);
        if let Some(buffer) = LOG_BUFFER.get() {
            buffer.push(LogMsg{
                log_type:LogType::ERROR,
                log_content:log_message
            });
        } else {
            eprintln!("LOG_BUFFER is not initialized yet.");
        }
    }

    fn timestamp() -> String{
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
        timestamp
    }

    pub fn get(size: usize) -> Vec<LogMsg> {
        if let Some(buffer) = LOG_BUFFER.get() {
            buffer.get(size)
        } else {
            vec![]
        }
    }

    // pub fn get_all() -> Vec<LogMsg> {
    //     if let Some(buffer) = LOG_BUFFER.get() {
    //         buffer.get_all()
    //     } else {
    //         vec![]
    //     }
    // }
}
