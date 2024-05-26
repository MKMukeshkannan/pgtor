use std::{
    error::Error,
    io::{stdout, Stdout},
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::components::{login, loading};

use super::state::{App, CurrentPage};

pub fn init() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();

    run_application(&mut terminal, &mut app)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn run_application(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {

    app.login();

    loop {
        match app.current_page {
            CurrentPage::Login => {
                login::run_component(terminal, app)?;
                return Ok(());
            },
            CurrentPage::Loading => loading::run_component(terminal, app)?,
        }

    }
}
