pub mod io;
pub mod layouts;
pub mod runtime;
pub mod widgets;

use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn centered_rect(percent_x: u16, popup_height: u16, frame_size: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((frame_size.height - popup_height) / 2),
                Constraint::Length(popup_height),
                Constraint::Length((frame_size.height - popup_height) / 2),
            ]
            .as_ref(),
        )
        .split(frame_size);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn centered_rect_with_margin(
    percent_x: u16,
    popup_height: u16,
    frame_size: Rect,
    margin: (Direction, u16),
) -> Rect {
    match margin.0 {
        Direction::Vertical => {
            let popup_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length((frame_size.height - popup_height) / 2),
                        Constraint::Length(popup_height),
                        Constraint::Length((frame_size.height - popup_height) / 2),
                    ]
                    .as_ref(),
                )
                .vertical_margin(margin.1)
                .split(frame_size);

            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage((100 - percent_x) / 2),
                        Constraint::Percentage(percent_x),
                        Constraint::Percentage((100 - percent_x) / 2),
                    ]
                    .as_ref(),
                )
                .split(popup_layout[1])[1]
        }
        Direction::Horizontal => {
            let popup_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length((frame_size.height - popup_height) / 2),
                        Constraint::Length(popup_height),
                        Constraint::Length((frame_size.height - popup_height) / 2),
                    ]
                    .as_ref(),
                )
                .split(frame_size);

            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage((100 - percent_x) / 2),
                        Constraint::Percentage(percent_x),
                        Constraint::Percentage((100 - percent_x) / 2),
                    ]
                    .as_ref(),
                )
                .horizontal_margin(margin.1)
                .split(popup_layout[1])[1]
        }
    }
}

pub mod prelude {
    pub use super::{io::*, runtime::TuiOpts, widgets::table::StatefulCmdsTable};
    pub use std::cell::RefCell;
    pub use tui::style::{Color, Modifier, Style};
}
