use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn list_layout<B: Backend>(frame: &mut Frame<B>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
        .split(frame.size())
}
