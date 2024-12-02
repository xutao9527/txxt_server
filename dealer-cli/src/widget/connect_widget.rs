use std::sync::{Arc, RwLock, Weak};

use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    app::app_data::{AppData, ConnectState},
    net::business_api::req_api::{self},
};

pub struct ConnectWidget {
    app_data: Weak<RwLock<AppData>>,
}

impl ConnectWidget {
    pub fn handle_events(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => {
                if let Some(app_data) = self.app_data.upgrade() {
                    if let Ok(mut data) = app_data.write() {
                        if data.client_select_id != 0 {
                            let select_id = data.client_select_id;
                            data.client_select_id = data.client_select_id - 1;
                            data.connects[select_id].state = ConnectState::Normal;
                            data.connects[select_id - 1].state = ConnectState::Selected;
                        }
                    }
                }
            }
            KeyCode::Down => {
                if let Some(app_data) = self.app_data.upgrade() {
                    if let Ok(mut data) = app_data.write() {
                        if data.client_select_id < 11 {
                            let select_id = data.client_select_id;
                            data.client_select_id = data.client_select_id + 1;
                            data.connects[select_id].state = ConnectState::Normal;
                            data.connects[select_id + 1].state = ConnectState::Selected;
                        }
                    }
                }
            }
            KeyCode::Enter => {
                if let Ok(_) = req_api::open() {
                    req_api::login();
                }
            }
            _ => {}
        }
    }
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
                .enumerate()
                .map(|(index, connect_info)| {
                    let con_view_str = match data.client_connected_id {
                        Some(pos) => {
                            if pos == index {
                                format!(" * Table .No : {}", connect_info.table_no)
                            } else {
                                format!("   Table .No : {}", connect_info.table_no)
                            }
                        }
                        None => {
                            format!("   Table .No : {}", connect_info.table_no)
                        }
                    };
                    match connect_info.state {
                        ConnectState::Selected => {
                            Line::from(con_view_str).style(Style::new().fg(Color::Green))
                        }
                        _ => match data.client_connected_id {
                            Some(pos) => {
                                if pos == index {
                                    Line::from(con_view_str).style(Style::new().fg(Color::Yellow))
                                } else {
                                    Line::from(con_view_str)
                                }
                            }
                            None => Line::from(con_view_str),
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
