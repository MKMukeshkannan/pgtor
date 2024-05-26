use core::time;
use std::{
    error::Error,
    io::{stdout, Stdout},
};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

use super::state::{App, DBCred};

pub fn init() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();

    let dbc: DBCred = DBCred {
        db_name: String::from("mk"),
        user_name: String::from("MUK"),
        password: String::from("!@$"),
        port: 32,
    };

    app.login(dbc);

    run_application(&mut terminal)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn run_application(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| {
            let area = f.size();
            f.render_widget(
                Paragraph::new("CSV Parser")
                    .style(
                        Style::new()
                            .fg(ratatui::style::Color::Red)
                            .bg(ratatui::style::Color::Yellow),
                    )
                    .centered()
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_style(Style::new().fg(ratatui::style::Color::Blue))
                            .border_type(BorderType::Double),
                    ),
                area,
            );
        })?;

        if event::poll(time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
