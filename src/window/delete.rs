use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{palette::tailwind::Palette, Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};

use crate::ssh::Host;

use super::{centered_rect, PopupWindow};

pub struct ShowData {
    host_index_to_delete: usize,
    host_to_delete: Host,
}

impl ShowData {
    pub fn new(host_index_to_delete: usize, host_to_delete: Host) -> Self {
        Self {
            host_index_to_delete,
            host_to_delete,
        }
    }
}

#[derive(Default)]
pub struct DeletePopupWindow {
    is_active: bool,

    selected_button_index: usize,
    show_data: Option<ShowData>,
}

impl PopupWindow for DeletePopupWindow {
    type ShowData = ShowData;

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn show(&mut self, data: Self::ShowData) {
        self.show_data = Some(ShowData::new(
            data.host_index_to_delete,
            data.host_to_delete,
        ));
        self.is_active = true;
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn render(&mut self, f: &mut Frame) {
        if let Some(show_data) = &self.show_data {
            let popup_area = centered_rect(50, 20, f.area());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .margin(1)
                .split(popup_area);

            let block = Block::bordered().border_type(BorderType::Rounded);
            // .border_style(Style::new().fg(Palette:));

            f.render_widget(Clear, popup_area);
            f.render_widget(block, popup_area);

            let question = Paragraph::new(Text::from(vec![Line::from(vec![Span::raw(format!(
                "Delete `{}` record?",
                show_data.host_to_delete.name
            ))])
            .bold()]))
            .alignment(Alignment::Center);

            f.render_widget(question, chunks[0]);

            let yes_style = if self.selected_button_index == 0 {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green)
            };

            let no_style = if self.selected_button_index == 1 {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red)
            };

            let buttons_line = Line::from(vec![
                Span::styled("  Yes  ", yes_style),
                Span::raw("   "),
                Span::styled("  No  ", no_style),
            ]);

            let buttons = Paragraph::new(Text::from(buttons_line)).alignment(Alignment::Center);

            f.render_widget(buttons, chunks[2]);
        }
    }
}

impl DeletePopupWindow {
    pub fn next(&mut self) {
        self.selected_button_index = 1;
    }

    pub fn previous(&mut self) {
        self.selected_button_index = 0;
    }
}
