use std::sync::{Arc, RwLock, Weak};

use btn::Button;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Widget},
};

use crate::app::app_data::AppData;

use super::util::{self, widget_util};

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

    use crate::log::log::SLog;
    pub enum State {
        Normal,
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
            let block = Block::bordered().borders(Borders::ALL);
            let mut paragraph = Paragraph::new(Text::raw(self.label.clone()))
                .block(block)
                .alignment(ratatui::layout::Alignment::Center);

            if self.label == "庄" {
                paragraph = paragraph.style(Style::new().fg(Color::Green)); // 背景色只会影响文本区域
                SLog::info("asdfasdf".into()); // 这里你可以在条件满足时做一些日志记录
            }
            paragraph.render(area, buf);
        }
    }
}
