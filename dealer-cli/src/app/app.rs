use std::sync::{Arc, RwLock, Weak};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    DefaultTerminal, Frame,
};

use crate::{
    log::log::SLog,
    widget::{
        control_widget::ControlWidget, info_widget::InfoWidget, log_widget::LogWidget,
        state_widget::StateWidget, view_widget::ViewWidget,
    },
};

use super::app_data::{self, AppData};

pub struct App {
    app_data: Weak<RwLock<AppData>>,
    log_widget: LogWidget,
}

impl App {
    pub fn new() -> Self {
        Self {
            app_data: Arc::downgrade(&AppData::singleton()),
            log_widget: LogWidget::default(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        loop {
            if let Some(app_data) = self.app_data.upgrade() {
                let data = app_data.read().unwrap();
                let should_exit_log = format!("App run should_exit [{}]", data.should_exit);
                SLog::info(should_exit_log);
                if data.should_exit {
                    break;
                }
            }
            terminal
                .draw(|frame| self.draw(frame))
                .expect("Failed to draw");
            self.handle_events();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let [frame_view, _] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        let [info, game, log, state] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(9),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(frame_view);
        let [game_view, game_control] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(18)]).areas(game);
        frame.render_widget(InfoWidget {}, info);
        frame.render_widget(ViewWidget {}, game_view);
        frame.render_widget(ControlWidget {}, game_control);
        frame.render_widget(&self.log_widget, log);
        frame.render_widget(StateWidget {}, state);
    }

    fn handle_events(&mut self) {
        if let Event::Key(key) = event::read().expect("failed to read event") {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                if let Some(app_data) = self.app_data.upgrade() {
                    let mut data = app_data.write().unwrap();
                    data.should_exit = true;
                    let should_exit_log =
                        format!("handle_events should_exit [{}]", data.should_exit);
                    SLog::err(should_exit_log);
                }
            }
        }
    }
}
