use crate::tui::widgets::table::StatefulCmdsTable;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

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
            Row::new(
                [
                    key.to_string(),
                    cmd.bin.to_string(),
                    aliases,
                    cmd.permissions.to_string(),
                ]
                .iter()
                .map(|c| Cell::from(c.clone())),
            )
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
