use app::app::App;

mod app;
mod log;
mod widget;

fn main() {
    let terminal = ratatui::init();
    let app = App::new(1000);
    app.run(terminal);
    ratatui::restore();
}
