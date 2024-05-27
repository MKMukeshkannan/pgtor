use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
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
        let form_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        let mut database_block = Block::new().borders(Borders::ALL).title("DATABASE");
        let mut port_block = Block::new().borders(Borders::ALL).title("PORT");
        let mut user_block = Block::new().borders(Borders::ALL).title("USERNAME");
        let mut password_block = Block::new().borders(Borders::ALL).title("PASSWORD");

        let active_style = Style::default().bg(ratatui::style::Color::Green);
        match self.currently_editing {
            CurrentlyEditing::DatabaseURL => database_block = database_block.style(active_style),
            CurrentlyEditing::Password => password_block = password_block.style(active_style),
            CurrentlyEditing::Port => port_block = port_block.style(active_style),
            CurrentlyEditing::UserName => user_block = user_block.style(active_style),
        }

        Paragraph::new(self.database_url.clone())
            .block(database_block)
            .render(form_layout[0], buf);
        Paragraph::new(self.port.clone())
            .block(port_block)
            .render(form_layout[1], buf);
        Paragraph::new(self.user_name.clone())
            .block(user_block)
            .render(form_layout[2], buf);
        Paragraph::new(self.password.clone())
            .block(password_block)
            .render(form_layout[3], buf);
    }
}
