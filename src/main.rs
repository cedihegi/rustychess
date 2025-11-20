use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{info, LevelFilter};
use ratatui::{prelude::CrosstermBackend, Terminal};
use rustychess::gui::app::App;
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::{
    fs::File,
    io::{self},
};

fn main() -> io::Result<()> {
    let _ = CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("app.log")?,
    )]);

    info!("initialized logging!");
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app_result = App::default().run(&mut terminal);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    app_result
}
