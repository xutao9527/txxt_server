use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::sync::{Arc, RwLock, Weak};

use crate::app::app_data::AppData;

pub struct ViewWidget {
    app_data: Weak<RwLock<AppData>>,
}

impl Default for ViewWidget {
    fn default() -> Self {
        ViewWidget {
            app_data: Arc::downgrade(&AppData::singleton()),
        }
    }
}

impl Widget for &ViewWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let log_block = Block::bordered().borders(Borders::ALL);
        let log_block = Paragraph::new(Text::raw("ViewWidget")).block(log_block);
        log_block.render(area, buf);
    }
}
