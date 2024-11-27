mod app;
use app::App;


fn main() {
    let terminal = ratatui::init();
    App::new().run(terminal);
    ratatui::restore();
}

