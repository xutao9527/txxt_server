use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    DefaultTerminal, Frame,
};

use crate::widget::{
    control_widget::ControlWidget, info_widget::InfoWidget, log_widget::LogWidget, state_widget::StateWidget, view_widget::ViewWidget
};

pub struct App {
    should_exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_exit: false }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        while !self.should_exit {
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
        frame.render_widget(LogWidget {}, log);
        frame.render_widget(StateWidget {}, state);
    }

    fn handle_events(&mut self) {
        if let Event::Key(key) = event::read().expect("failed to read event") {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.should_exit = true;
            }
        }
    }
}
