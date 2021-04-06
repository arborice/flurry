use crate::tui::widgets::table::StatefulCmdsTable;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

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

pub fn full_win_layout<B: Backend>(frame: &mut Frame<B>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
        .split(frame.size())
}

pub fn table_layout<B: Backend>(
    table: &mut StatefulCmdsTable,
    frame: &mut Frame<B>,
    layout: Vec<Rect>,
    selected_style: Style,
) {
    let header_cells = ["Key", "Cmd", "Aliases", "Permissions"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);
    frame.render_stateful_widget(
        Table::new(table.cmds.borrow().iter().map(|(key, cmd)| {
            use rkyv::core_impl::ArchivedOption;
            let aliases = match &cmd.aliases {
                ArchivedOption::Some(aliases) => aliases.join(", "),
                ArchivedOption::None => String::new(),
            };

            Row::new(std::array::IntoIter::new([
                Cell::from(key.as_str()),
                Cell::from(cmd.bin.as_str()),
                Cell::from(aliases),
                Cell::from(cmd.permissions.as_ref()),
            ]))
            .height(1)
        }))
        .highlight_style(selected_style)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Your Flurry Generated Commands"),
        )
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(45),
            Constraint::Percentage(15),
        ]),
        layout[0],
        &mut table.state,
    )
}
