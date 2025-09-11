pub mod delete;

pub use delete::DeletePopupWindow;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub trait PopupWindow {
    type ShowData;

    fn is_active(&self) -> bool;
    fn show(&mut self, data: Self::ShowData);
    fn hide(&mut self);
    fn toggle(&mut self);
    fn render(&mut self, f: &mut Frame);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
