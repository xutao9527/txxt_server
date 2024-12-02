use std::sync::{Arc, RwLock, Weak};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::app_data::AppData;
pub struct ConnectWidget {
    app_data: Weak<RwLock<AppData>>,
}

impl Default for ConnectWidget {
    fn default() -> Self {
        ConnectWidget {
            app_data: Arc::downgrade(&AppData::singleton()),
        }
    }
}

impl Widget for &ConnectWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized,
    {
        let log_block = Block::bordered().borders(Borders::ALL);
        let log_block = Paragraph::new(Text::raw("ConnectWidget")).block(log_block);
        log_block.render(area, buf);
    }
}
