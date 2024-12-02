use std::sync::{Arc, RwLock, Weak};

use btn::Button;
use futures_util::SinkExt;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Widget},
};

use crate::app::app_data::AppData;

use super::util::widget_util;

pub struct ControlWidget {
    app_data: Weak<RwLock<AppData>>,
    buttons: Vec<Vec<Button>>,
    position: (u16, u16),
}

impl Default for ControlWidget {
    fn default() -> Self {
        let buttons = vec![
            vec![
                Button::new("庄".into(), (0, 0)).select(),
                Button::new("庄-庄对".into(), (0, 1)),
                Button::new("庄-闲对".into(), (0, 2)),
                Button::new("庄-庄闲对".into(), (0, 3)),
            ],
            vec![
                Button::new("闲".into(), (1, 0)),
                Button::new("闲-庄对".into(), (1, 1)),
                Button::new("闲-闲对".into(), (1, 2)),
                Button::new("闲-庄闲对".into(), (1, 3)),
            ],
            vec![
                Button::new("和".into(), (2, 0)),
                Button::new("和-庄对".into(), (2, 1)),
                Button::new("和-闲对".into(), (2, 2)),
                Button::new("和-庄闲对".into(), (2, 3)),
            ],
            vec![
                Button::new("六".into(), (3, 0)),
                Button::new("六-庄对".into(), (3, 1)),
                Button::new("六-闲对".into(), (3, 2)),
                Button::new("六-庄闲对".into(), (3, 3)),
            ],
        ];
        ControlWidget {
            position: (0, 0),
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
        let btn_layout = widget_util::center(area, Constraint::Length(48), Constraint::Length(12));
        let btn_layout = Layout::horizontal([
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(0),
        ])
        .split(btn_layout)
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
        Block::bordered().borders(Borders::ALL).render(area, buf);
    }
}

mod btn {
    use ratatui::{
        style::{Color, Style},
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
        position: (u16, u16),
    }

    impl Button {
        pub fn new(lable: String, position: (u16, u16)) -> Self {
            Button {
                label: lable,
                state: State::Normal,
                position,
            }
        }

        pub fn select(mut self) -> Self {
            self.state = State::Selected;
            self
        }
    }

    impl Widget for &Button {
        fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized,
        {
            let block = Block::bordered().borders(Borders::ALL);
            let mut paragraph = Paragraph::new(Text::raw(self.label.clone()))
                .block(block)
                .alignment(ratatui::layout::Alignment::Center);

            match self.state {
                State::Selected => {
                    paragraph = paragraph.style(Style::new().fg(Color::Green)); // 背景色只会影响文本区域
                }
                _ => {}
            }
            paragraph.render(area, buf);
        }
    }
}
