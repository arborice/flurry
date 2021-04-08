use crate::prelude::*;

use std::{array, cell::RefCell};
use tui::widgets::TableState;

type CmdsRef<'cmds> = &'cmds RefCell<&'cmds mut HashMap<String, GeneratedCommand>>;

pub struct StatefulCmdsTable<'cmds> {
    pub cmds: CmdsRef<'cmds>,
    pub state: TableState,
    pub request_exit: bool,
    pub selected_indices: Vec<usize>,
    pub key_cache: HashMap<usize, String>,
    header_style: Style,
    selection_style: Style,
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
        let borrowed_cmds = cmds.borrow();
        let key_cache = borrowed_cmds.keys().enumerate().fold(
            HashMap::with_capacity(borrowed_cmds.len()),
            |mut map, (i, key)| (map.insert(i, key.clone()), map).1,
        );

        StatefulCmdsTable {
            cmds,
            key_cache,
            selected_indices: Vec::new(),
            request_exit: false,
            state: TableState::default(),
            header_style: Style::default(),
            selection_style: Style::default(),
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

    pub fn with_selection_style(mut self, style: Style) -> Self {
        self.selection_style = style;
        self
    }

    pub fn update_cache(&mut self) {
        let borrowed_cmds = self.cmds.borrow();
        self.key_cache = borrowed_cmds.keys().enumerate().fold(
            HashMap::with_capacity(borrowed_cmds.len()),
            |mut map, (i, key)| (map.insert(i, key.clone()), map).1,
        );
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
        let header_cells = array::IntoIter::new(["Key", "Cmd", "Aliases", "Permissions"])
            .map(|h| Cell::from(h).style(self.header_style));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        frame.render_stateful_widget(
            Table::new(
                self.cmds
                    .borrow()
                    .iter()
                    .enumerate()
                    .map(|(i, (key, cmd))| {
                        let style = if self.selected_indices.contains(&i) {
                            self.rm_selection_style
                        } else {
                            Default::default()
                        };

                        Row::new(array::IntoIter::new([
                            Cell::from(key.as_str()),
                            Cell::from(cmd.bin.as_str()),
                            Cell::from(
                                cmd.aliases
                                    .as_ref()
                                    .and_then(|a| Some(a.join(", ")))
                                    .unwrap_or_default(),
                            ),
                            Cell::from(cmd.permissions.as_ref()),
                        ]))
                        .style(style)
                        .height(1)
                    }),
            )
            .highlight_style(self.selection_style)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Your Flurry Generated Commands"),
            )
            .widths(&[
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(55),
                Constraint::Percentage(15),
            ]),
            layout[0],
            &mut self.state,
        )
    }
}
