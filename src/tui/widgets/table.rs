use crate::{prelude::*, tui::runtime::TuiOpts};

use std::cell::RefCell;
use tui::widgets::TableState;

use rkyv::std_impl::ArchivedString;
type CmdsRef<'cmds> =
    &'cmds RefCell<&'cmds Vec<(&'cmds ArchivedString, &'cmds ArchivedGeneratedCommand)>>;

pub struct StatefulCmdsTable<'cmds> {
    pub cmds: CmdsRef<'cmds>,
    pub opts: TuiOpts,
    pub state: TableState,
    pub selected_indices: Vec<usize>,
    pub request_exit: bool,
}

impl<'cmds> StatefulCmdsTable<'cmds> {
    pub fn with_items(cmds: CmdsRef<'cmds>) -> StatefulCmdsTable<'cmds> {
        let selected_indices = Vec::new();
        StatefulCmdsTable {
            cmds: cmds,
            selected_indices,
            state: TableState::default(),
            opts: Default::default(),
            request_exit: false,
        }
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
}
