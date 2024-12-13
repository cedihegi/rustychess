use rustychess::gui::app::App;
use std::io::{self};

fn main() -> io::Result<()> {
    let mut tui = ratatui::init();
    let app_result = App::default().run(&mut tui);
    ratatui::restore();
    app_result
}
