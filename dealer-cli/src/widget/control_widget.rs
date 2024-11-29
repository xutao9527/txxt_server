use std::sync::{Arc, RwLock, Weak};

use btn::Button;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::app::app_data::AppData;

pub struct ControlWidget {
    app_data: Weak<RwLock<AppData>>,
    buttons: Vec<Vec<Button>>,
}

impl Default for ControlWidget {
    fn default() -> Self {
        let buttons = vec![
            vec![
                Button::new("庄".into()),
                Button::new("庄-庄对".into()),
                Button::new("庄-闲对".into()),
                Button::new("庄-庄闲对".into()),
            ],
            vec![
                Button::new("闲".into()),
                Button::new("闲-庄对".into()),
                Button::new("闲-闲对".into()),
                Button::new("闲-庄闲对".into()),
            ],
            vec![
                Button::new("和".into()),
                Button::new("和-庄对".into()),
                Button::new("和-闲对".into()),
                Button::new("和-庄闲对".into()),
            ],
            vec![
                Button::new("六".into()),
                Button::new("六-庄对".into()),
                Button::new("六-闲对".into()),
                Button::new("六-庄闲对".into()),
            ],
        ];
        ControlWidget {
            app_data: Arc::downgrade(&AppData::singleton()),
            buttons,
        }
    }
}

impl Widget for &ControlWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let btn_layout = Layout::horizontal([
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(0),
        ])
        .split(area)
        .iter()
        .map(|h_chunk| {
            Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(*h_chunk)
            .to_vec()
        })
        .collect::<Vec<Vec<Rect>>>();

        for (i, row) in self.buttons.iter().enumerate() {
            for (j, chunk) in row.iter().enumerate() {
                chunk.render(btn_layout[i][j], buf);
            }
        }
    }
}

mod btn {
    use ratatui::{
        text::Text,
        widgets::{Block, Borders, Paragraph, Widget},
    };
    pub enum State {
        Normal,
        Selected,
    }

    pub struct Button {
        label: String,
        state: State,
    }

    impl Button {
        pub fn new(lable: String) -> Self {
            Button {
                label: lable,
                state: State::Normal,
            }
        }
    }

    impl Widget for &Button {
        fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized,
        {
            let log_block = Block::bordered().borders(Borders::ALL);
            let log_block = Paragraph::new(Text::raw(self.label.clone()))
                .block(log_block)
                .alignment(ratatui::layout::Alignment::Center);
            log_block.render(area, buf);
        }
    }
}
