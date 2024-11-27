use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
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
        let [info, data, log, status] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(frame.area());

       
        let info_block = Block::bordered().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);

        // let info_block = Paragraph::new(Text::raw("info_block")).block(info_block);

        let data_block = Block::bordered().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);
        // let data_block = Paragraph::new(Text::raw("data_block")).block(data_block);

        let log_block = Block::bordered().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);
        // let log_block = Paragraph::new(Text::raw("log_block")).block(log_block);

        let status_block = Block::bordered();
        // let status_block = Paragraph::new(Text::raw("status_block")).block(status_block);

        frame.render_widget(info_block, info);
        frame.render_widget(data_block, data);
        frame.render_widget(log_block, log);
        frame.render_widget(status_block, status);
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
