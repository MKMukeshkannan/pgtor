use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

#[derive(Debug)]
pub enum EditMode {
    Normal,
    Insert,
}

pub enum CurrentlyEditing {
    DatabaseURL,
    Port,
    UserName,
    Password,
}

pub struct ConnectionForm {
    pub mode: EditMode,
    pub currently_editing: CurrentlyEditing,
    pub database_url: String,
    pub port: String,
    pub user_name: String,
    pub password: String,
}

impl ConnectionForm {
    pub fn new() -> ConnectionForm {
        ConnectionForm {
            mode: EditMode::Normal,
            currently_editing: CurrentlyEditing::DatabaseURL,
            database_url: String::from(""),
            port: String::from(""),
            user_name: String::from(""),
            password: String::from(""),
        }
    }

    pub fn next(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::DatabaseURL => self.currently_editing = CurrentlyEditing::Port,
            CurrentlyEditing::Port => self.currently_editing = CurrentlyEditing::UserName,
            CurrentlyEditing::UserName => self.currently_editing = CurrentlyEditing::Password,
            CurrentlyEditing::Password => self.currently_editing = CurrentlyEditing::DatabaseURL,
        }
    }

    pub fn prev(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::DatabaseURL => self.currently_editing = CurrentlyEditing::Password,
            CurrentlyEditing::Port => self.currently_editing = CurrentlyEditing::DatabaseURL,
            CurrentlyEditing::UserName => self.currently_editing = CurrentlyEditing::Port,
            CurrentlyEditing::Password => self.currently_editing = CurrentlyEditing::UserName,
        }
    }
}

impl Widget for &mut ConnectionForm {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let centered_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(1),
                Constraint::Percentage(50),
                Constraint::Min(1),
            ])
            .split(area);

        let form_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(centered_rect[1]);

        let mut database_block = Block::new()
            .borders(Borders::ALL)
            .padding(Padding::vertical(0))
            .border_type(BorderType::Rounded)
            .title("DATABASE URL").padding(Padding::horizontal(1));
        let mut port_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("PORT").padding(Padding::horizontal(1));
        let mut user_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("USERNAME").padding(Padding::horizontal(1));
        let mut password_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("PASSWORD").padding(Padding::horizontal(1));

        let active_style = match self.mode {
            EditMode::Normal => Style::new().green(),
            EditMode::Insert => Style::new().red(),
        };
        match self.currently_editing {
            CurrentlyEditing::DatabaseURL => {
                database_block = database_block.border_style(active_style)
            }
            CurrentlyEditing::Password => {
                password_block = password_block.border_style(active_style)
            }
            CurrentlyEditing::Port => port_block = port_block.border_style(active_style),
            CurrentlyEditing::UserName => user_block = user_block.border_style(active_style),
        }

        Paragraph::new("FILL IN THE DETAILS TO CONTINUE").centered().render(form_layout[0], buf);
        Paragraph::new(self.database_url.clone())
            .block(database_block)
            .render(form_layout[1], buf);
        Paragraph::new(self.port.clone())
            .block(port_block)
            .render(form_layout[2], buf);
        Paragraph::new(self.user_name.clone())
            .block(user_block)
            .render(form_layout[3], buf);
        Paragraph::new(self.password.clone())
            .block(password_block)
            .render(form_layout[4], buf);
    }
}
