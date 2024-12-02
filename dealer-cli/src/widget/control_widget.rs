use std::sync::{Arc, RwLock, Weak};

use btn::Button;
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Widget},
};

use crate::{app::app_data::AppData, log::log::SLog};

use super::util::widget_util;

pub struct ControlWidget {
    app_data: Weak<RwLock<AppData>>,
    buttons: Vec<Vec<Button>>,
    position: (u16, u16),
}

impl Default for ControlWidget {
    fn default() -> Self {
        let mut default_btn = Button::new("庄".into());
        default_btn.select();
        let buttons = vec![
            vec![
                default_btn,
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
            position: (0, 0),
            app_data: Arc::downgrade(&AppData::singleton()),
            buttons,
        }
    }
}

impl ControlWidget {
    pub fn handle_events(&mut self, code: KeyCode) {
        match code {
            KeyCode::Left => {
                let (x, y) = self.position;
                if x != 0 {
                    self.position = (x - 1, y);
                    self.buttons[x as usize][y as usize].deselect();
                    self.buttons[(x - 1) as usize][y as usize].select();
                }
                SLog::info(format!("Control input [{}] [{:?}]", code, self.position));
            }
            KeyCode::Right => {
                let (x, y) = self.position;
                if x + 1 < 4 {
                    self.position = (x + 1, y);
                    self.buttons[x as usize][y as usize].deselect();
                    self.buttons[(x + 1) as usize][y as usize].select();
                }
                SLog::info(format!("Control input [{}] [{:?}]", code, self.position));
            }
            KeyCode::Up => {
                let (x, y) = self.position;
                if y != 0 {
                    self.position = (x, y - 1);
                    self.buttons[x as usize][y as usize].deselect();
                    self.buttons[x as usize][(y - 1) as usize].select();
                }

                SLog::info(format!("Control input [{}] [{:?}]", code, self.position));
            }
            KeyCode::Down => {
                let (x, y) = self.position;
                if y + 1 < 4 {
                    self.position = (x, y + 1);
                    self.buttons[x as usize][y as usize].deselect();
                    self.buttons[x as usize][(y + 1) as usize].select();
                }
                SLog::info(format!("Control input [{}] [{:?}]", code, self.position));
            }
            _ => {}
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
       
    }

    impl Button {
        pub fn new(lable: String) -> Self {
            Button {
                label: lable,
                state: State::Normal,
            }
        }

        pub fn deselect(&mut self) {
            self.state = State::Normal;
        }

        pub fn select(&mut self) {
            self.state = State::Selected;
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
