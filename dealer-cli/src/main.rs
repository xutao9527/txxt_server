use tui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::line,
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};
use std::io;
use crossterm::{event, execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启用终端的原始模式
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; // 进入备用屏幕模式
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 渲染界面
    let res = run_app(&mut terminal);

    // 恢复终端
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("应用运行时出错: {:?}", err);
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            // 创建一个垂直布局
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                        .as_ref(),
                )
                .split(f.size());

            // 表格数据
            let rows = vec![
                Row::new(vec!["编号", "姓名", "分数"]).style(Style::default().fg(Color::Yellow)),
                Row::new(vec!["1", "Alice", "85"]),
                Row::new(vec!["2", "Bob", "92"]),
                Row::new(vec!["3", "Charlie", "78"]),
            ];

            // 表格
            let table = Table::new(rows)
                .header(Row::new(vec!["列1", "列2", "列3"]).style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
                .block(Block::default().title("示例表格").borders(Borders::ALL))
                .widths(&[
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length(10),
                ])
                .column_spacing(2);

            // 渲染表格
            f.render_widget(table, chunks[1]);
        })?;

        // 检测退出事件
        if event::poll(std::time::Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') {
                    return Ok(()); // 按 `q` 键退出
                }
            }
        }
    }
}
