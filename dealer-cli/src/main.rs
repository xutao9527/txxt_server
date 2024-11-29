use app::app::App;
use log::log::SLog;

mod app;
mod log;
mod widget;

fn main() {
    SLog::init(1000);
    SLog::err("1".to_string());
    SLog::info("2".to_string());
    SLog::info("3".to_string());
    let terminal = ratatui::init();
    let app = App::new();
    app.run(terminal);
    ratatui::restore();
}
