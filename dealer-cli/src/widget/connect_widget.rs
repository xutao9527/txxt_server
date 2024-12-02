use std::sync::{Arc, RwLock, Weak};

use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph, Widget}
};

use crate::app::app_data::{AppData, ConnectState};

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
        if let Some(app_data) = self.app_data.upgrade() {
            let data = app_data.read().unwrap();
            let connects = &data.connects;
            let connects_view = connects
                .iter()
                .map(|c| {
                    match c.state {
                        ConnectState::Selected => {
                            Line::from(format!(" Table .No : {}", c.table_no))
                            .style(Style::new().fg(Color::Green))
                        },
                        ConnectState::Connected => {
                            Line::from(format!(" Table .No : {} Conected", c.table_no))
                            .style(Style::new().fg(Color::Yellow))
                        },
                        _ => {
                            Line::from(format!(" Table .No : {}", c.table_no))
                        },
                    }
                    
                })
                .collect::<Vec<Line>>();
            let paragraph = Paragraph::new(connects_view)
                .gray()
                .block(Block::bordered().borders(Borders::ALL));
            paragraph.render(area, buf);
        }

      
    }
}
