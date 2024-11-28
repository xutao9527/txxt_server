use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct ViewWidget {}

impl Widget for ViewWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let log_block = Block::bordered().borders(Borders::ALL);
        let log_block = Paragraph::new(Text::raw("ViewWidget")).block(log_block);
        log_block.render(area, buf);
    }
}
