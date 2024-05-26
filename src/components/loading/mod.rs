use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use std::{error::Error, io::Stdout};

use crate::app::state::{App, CurrentPage};

pub fn run_component(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    while app.is_logged {
        terminal.draw(|frame| frame.render_widget(Paragraph::new("LOADIN"), frame.size()))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e') {
                    app.current_page = CurrentPage::Login;
                    break;
                }
            }
        }
    }
    Ok(())
}
