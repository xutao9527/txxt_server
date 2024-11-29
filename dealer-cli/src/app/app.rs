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

use super::app_data::AppData;

pub struct App {
    app_data: Weak<RwLock<AppData>>,
    info_widget:InfoWidget,
    view_widget:ViewWidget,
    control_widget:ControlWidget,
    log_widget: LogWidget,
    state_widget:StateWidget,
}

impl App {
    pub fn new(log_cache: usize) -> Self {
        SLog::init(log_cache);
        Self {
            app_data: Arc::downgrade(&AppData::singleton()),
            info_widget:InfoWidget::default(),
            view_widget:ViewWidget::default(),
            control_widget:ControlWidget::default(),
            log_widget: LogWidget::default(),
            state_widget:StateWidget::default(),
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
            Constraint::Length(20),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(frame_view);
        let [game_view, game_control] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(60)]).areas(game);
        frame.render_widget(&self.info_widget, info);
        frame.render_widget(&self.view_widget, game_view);
        frame.render_widget(&self.control_widget, game_control);
        frame.render_widget(&self.log_widget, log);
        frame.render_widget(&self.state_widget, state);
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
