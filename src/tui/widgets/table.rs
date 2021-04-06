use crate::prelude::*;

use std::cell::RefCell;
use tui::widgets::TableState;

type CmdsRef<'cmds> = &'cmds RefCell<&'cmds mut HashMap<String, GeneratedCommand>>;

pub struct StatefulCmdsTable<'cmds> {
    pub cmds: CmdsRef<'cmds>,
    pub state: TableState,
    pub request_exit: bool,
    pub selected_indices: Vec<usize>,
    header_style: Style,
    rm_selection_style: Style,
}

use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

impl<'cmds> StatefulCmdsTable<'cmds> {
    pub fn with_items(cmds: CmdsRef<'cmds>) -> StatefulCmdsTable<'cmds> {
        let selected_indices = Vec::new();
        StatefulCmdsTable {
            cmds,
            selected_indices,
            state: TableState::default(),
            request_exit: false,
            header_style: Style::default(),
            rm_selection_style: Style::default(),
        }
    }

    pub fn with_header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }

    pub fn with_rm_style(mut self, style: Style) -> Self {
        self.rm_selection_style = style;
        self
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.cmds.borrow().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.cmds.borrow().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index))
    }

    pub fn show_layout<B: Backend>(&mut self, frame: &mut Frame<B>, layout: Vec<Rect>) {
        let header_cells = ["Key", "Cmd", "Aliases", "Permissions"]
            .iter()
            .map(|h| Cell::from(*h).style(self.header_style));
        let header = Row::new(header_cells).height(1).bottom_margin(1);
        frame.render_stateful_widget(
            Table::new(self.cmds.borrow().iter().map(|(key, cmd)| {
                let aliases = match &cmd.aliases {
                    Some(aliases) => aliases.join(", "),
                    None => String::new(),
                };

                Row::new(std::array::IntoIter::new([
                    Cell::from(key.as_str()),
                    Cell::from(cmd.bin.as_str()),
                    Cell::from(aliases),
                    Cell::from(cmd.permissions.as_ref()),
                ]))
                .height(1)
            }))
            .highlight_style(self.rm_selection_style)
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
            &mut self.state,
        )
    }
}
