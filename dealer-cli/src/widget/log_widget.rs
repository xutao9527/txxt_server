use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Masked, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState, StatefulWidget, Widget},
};

#[derive(Default)]
pub struct LogWidget {
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
}

impl Widget for LogWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let s =
            "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
        let mut long_line = s.repeat(usize::from(area.width) / s.len() + 4);
        long_line.push('\n');
        let log_block = Block::bordered().borders(Borders::ALL);
        let log_block = Paragraph::new(Text::raw("LogWidget")).block(log_block);
        log_block.render(area, buf);
        let text = vec![
            Line::from("1 This is a line "),
            Line::from("2 This is a line   ".red()),
            Line::from("3 This is a line".on_dark_gray()),
            Line::from("4 This is a longer line".crossed_out()),
            Line::from("5 This is a line".reset()),
            Line::from("6 This is a line "),
            Line::from("7 This is a line   ".red()),
            Line::from("8 This is a line".on_dark_gray()),
            Line::from("9 This is a longer line".crossed_out()),
            Line::from("10 This is a line".reset()),
            Line::from("11 This is a line "),
            Line::from("12 This is a line   ".red()),
            Line::from("13 This is a line".on_dark_gray()),
            Line::from("14 This is a longer line".crossed_out()),
            Line::from("15 This is a line".reset()),
            Line::from("16 This is a line "),
            Line::from("17 This is a line   ".red()),
            Line::from("18 This is a line".on_dark_gray()),
            Line::from("19 This is a longer line".crossed_out()),
            Line::from("20 This is a line"),
        ];
        let line_total = text.len() as u16;
        let line_view = area.height - 2;
        let line_scroll = if line_total >= line_view {
            line_total - line_view + 1
        } else {
            0
        };
        let paragraph = Paragraph::new(text.clone())
            .gray()
            .block(Block::bordered().borders(Borders::ALL))
            .scroll((line_scroll, 0));
        paragraph.render(area, buf);
    }
}
