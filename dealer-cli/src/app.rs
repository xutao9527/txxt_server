use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    widgets::Block,
    DefaultTerminal, Frame,
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
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
        let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right);
        frame.render_widget(Block::bordered().title("Left Block"), left);
        frame.render_widget(Block::bordered().title("Top Right Block"), top_right);
        frame.render_widget(Block::bordered().title("Bottom Right Block"), bottom_right);
    }

    fn handle_events(&mut self) {
        if let Event::Key(key) = event::read().expect("failed to read event") {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.should_exit = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::App;

   

    #[test]
    fn test_terminal() {
        let terminal = ratatui::init();
        App::new().run(terminal);
        ratatui::restore();
    }
}
