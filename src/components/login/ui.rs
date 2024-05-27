use super::{
    form::{ConnectionForm, EditMode},
    utils::PGTOR,
};
use crate::app::state::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

pub fn ui(frame: &mut Frame, _app: &mut App, connection_form: &mut ConnectionForm) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(1), Constraint::Length(3)])
        .split(frame.size());

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(10), Constraint::Min(1)])
        .split(outer_layout[0]);

    frame.render_widget(Paragraph::new(PGTOR).centered(), main_layout[0]);

    let current_editing_mode = match connection_form.mode {
        EditMode::Normal => {
            Span::styled(" NORMAL ", Style::default().fg(ratatui::style::Color::Gray)).bg(ratatui::style::Color::Green)
        }
        EditMode::Insert => Span::styled(" INSERT ", Style::default().fg(ratatui::style::Color::Gray).bg(ratatui::style::Color::Red)),
    };

    frame.render_widget(connection_form, main_layout[1]);

    let footer_block = Block::new().borders(Borders::all()).padding(Padding::horizontal(1));
    frame.render_widget(
        Paragraph::new(current_editing_mode).block(footer_block),
        outer_layout[1],
    );
}
