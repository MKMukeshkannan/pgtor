mod form;
mod ui;
mod utils;

use crate::app::state::App;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io::Stdout};
use ui::ui;

use form::{ConnectionForm, CurrentlyEditing, EditMode};

pub fn run_component(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    let mut connection_form = ConnectionForm::new();

    loop {
        terminal.draw(|f| ui(f, app, &mut connection_form))?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match connection_form.mode {
                EditMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('i') => connection_form.mode = EditMode::Insert,
                    KeyCode::Char('j') => connection_form.next(),
                    KeyCode::Char('k') => connection_form.prev(),
                    _ => {}
                },
                EditMode::Insert => match key.code {
                    KeyCode::Esc => connection_form.mode = EditMode::Normal,

                    KeyCode::Backspace => match &connection_form.currently_editing {
                        CurrentlyEditing::DatabaseURL => {
                            connection_form.database_url.pop();
                        }
                        CurrentlyEditing::Password => {
                            connection_form.password.pop();
                        }
                        CurrentlyEditing::Port => {
                            connection_form.port.pop();
                        }
                        CurrentlyEditing::UserName => {
                            connection_form.user_name.pop();
                        }
                    },

                    KeyCode::Char(value) => match &connection_form.currently_editing {
                        CurrentlyEditing::DatabaseURL => connection_form.database_url.push(value),
                        CurrentlyEditing::Password => connection_form.password.push(value),
                        CurrentlyEditing::Port => connection_form.port.push(value),
                        CurrentlyEditing::UserName => connection_form.user_name.push(value),
                    },
                    _ => {}
                },
            };
        }
    }

    Ok(())
}
