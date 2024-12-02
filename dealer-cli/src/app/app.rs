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
use crate::widget::connect_widget::ConnectWidget;

use super::app_data::{AppData, TerminalMode};

pub struct App {
    app_data: Weak<RwLock<AppData>>,
    info_widget: InfoWidget,
    view_widget: ViewWidget,
    control_widget: ControlWidget,
    connect_widget:ConnectWidget,
    log_widget: LogWidget,
    state_widget: StateWidget,
}

impl App {
    pub fn new(log_cache: usize) -> Self {
        SLog::init(log_cache);
        Self {
            app_data: Arc::downgrade(&AppData::singleton()),
            info_widget: InfoWidget::default(),
            view_widget: ViewWidget::default(),
            control_widget: ControlWidget::default(),
            connect_widget: ConnectWidget::default(),
            log_widget: LogWidget::default(),
            state_widget: StateWidget::default(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        loop {
            if let Some(app_data) = self.app_data.upgrade() {
                let data = app_data.read().unwrap();
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
        let [v1, v2, v3, v4] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(14),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(frame_view);
        let [game_view, game_control] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(50)]).areas(v2);

        let [log_view, connect_view] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(22)]).areas(v3);

        frame.render_widget(&self.info_widget, v1);
        frame.render_widget(&self.view_widget, game_view);
        frame.render_widget(&self.control_widget, game_control);
        frame.render_widget(&self.log_widget, log_view);
        frame.render_widget(&self.connect_widget, connect_view);
        frame.render_widget(&self.state_widget, v4);
    }

    fn handle_events(&mut self) {
        if let Event::Key(key) = event::read().expect("failed to read event") {
            if key.kind == KeyEventKind::Press {
                if let Some(app_data) = self.app_data.upgrade() {
                    
                            
                    match key.code {
                        KeyCode::Char('s') => {
                            app_data.write().unwrap().terminal_mode = TerminalMode::Control;
                            SLog::info(format!("App input [{}]", key.code));
                            return;
                        },
                        KeyCode::Char('c') => {
                            app_data.write().unwrap().terminal_mode = TerminalMode::Connect;
                            SLog::info(format!("App input [{}]", key.code));
                            return;
                        },
                        KeyCode::Char('q') => {
                            app_data.write().unwrap().should_exit = true;
                            SLog::info(format!("App input [{}]", key.code));
                            return;
                        },
                        _ => {}
                    }
                    let data = app_data.read().unwrap();
                    match data.terminal_mode {
                        TerminalMode::Control => {
                            drop(data);
                            self.control_widget.handle_events(key.code);
                        },
                        TerminalMode::Connect => {
                            drop(data);
                            self.connect_widget.handle_events(key.code);
                        }
                        _ => {},
                    }
                }
            }
        }
        
    }
}
