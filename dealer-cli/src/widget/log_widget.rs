use std::sync::{Arc, RwLock, Weak};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    app::app_data::AppData,
    log::log::{LogMsg, LogType, SLog},
};

pub struct LogWidget {
    app_data: Weak<RwLock<AppData>>,
}

impl Default for LogWidget {
    fn default() -> Self {
        LogWidget {
            app_data: Arc::downgrade(&AppData::singleton()),
        }
    }
}

impl Widget for &LogWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let logs: Vec<LogMsg> = SLog::get((area.height - 3).into());
        let logs_view = logs
            .into_iter()
            .map(|log| match log.log_type {
                LogType::INFO => Line::from(log.log_content),
                LogType::ERROR => Line::from(log.log_content.red()),
            })
            .collect::<Vec<_>>();
        let paragraph = Paragraph::new(logs_view)
            .gray()
            .block(Block::bordered().borders(Borders::ALL));
        paragraph.render(area, buf);
    }
}
