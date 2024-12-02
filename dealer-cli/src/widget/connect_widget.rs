use std::sync::{Arc, RwLock, Weak};

use dealer::protocol::{definition::packet_request::PacketRequest, handler::PacketType, payload::{login::LoginReq, PacketPayload}};
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
    log::log::SLog, net::business_api::login,
};

pub struct ConnectWidget {
    app_data: Weak<RwLock<AppData>>,
    position: usize,
}

impl ConnectWidget {
    pub fn handle_events(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => {
                if self.position != 0 {
                    if let Some(data) = self.app_data.upgrade() {
                        let connects = &mut data.write().unwrap().connects;
                        connects[self.position].state = ConnectState::Normal;
                        self.position -= 1;
                        connects[self.position].state = ConnectState::Selected;
                    }
                }
            }
            KeyCode::Down => {
                if self.position < 11 {
                    if let Some(data) = self.app_data.upgrade() {
                        let connects = &mut data.write().unwrap().connects;
                        connects[self.position].state = ConnectState::Normal;
                        self.position += 1;
                        connects[self.position].state = ConnectState::Selected;
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(data) = self.app_data.upgrade() {
                    let data: &mut std::sync::RwLockWriteGuard<'_, AppData> = &mut data.write().unwrap();

                    //data.connected = Option::from(self.position);

                    let connect_info = data.connects[self.position].clone();

                    match data.dealer_client.open() {
                        Ok(_) => {
                            login(data,connect_info);
                            // let login_req = PacketRequest {
                            //     packet_type: PacketType::Login,
                            //     packet_payload: PacketPayload::LoginReq(LoginReq {
                            //         user_name: data.connects[self.position].table_no.clone(),
                            //         pass_word: data.connects[self.position].password.clone(),
                            //     }),
                            // };
                            // let _ = data.dealer_client.send(login_req);
                        },
                        Err(_) => {},
                    }
                    SLog::info(format!("Connect input [{}] [{:?}]", code, self.position));
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
            position: 0,
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
                .map(|(i, c)| {
                    let con_view_str = match data.connected {
                        Some(pos) => {
                            if pos == i {
                                format!(" * Table .No : {}", c.table_no)
                            } else {
                                format!("   Table .No : {}", c.table_no)
                            }
                        }
                        None => {
                            format!("   Table .No : {}", c.table_no)
                        }
                    };
                    match c.state {
                        ConnectState::Selected => {
                            Line::from(con_view_str).style(Style::new().fg(Color::Green))
                        }
                        _ => match data.connected {
                            Some(pos) => {
                                if pos == i {
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
