use ratatui::{
    crossterm::event::{self, Event},
    text::Text,
    DefaultTerminal, Frame,
};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, mut terminal: DefaultTerminal) {
        loop {
            terminal
                .draw(|frame| self.draw(frame))
                .expect("Failed to draw");

            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(Text::raw("Hello World!"), frame.area())
    }
}
