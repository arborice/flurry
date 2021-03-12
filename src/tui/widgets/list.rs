use std::cell::RefCell;
use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub trait ListEntry {
    fn as_entry(&self) -> String;
    fn as_context(&self) -> &str;
}

impl<T: ListEntry> ListEntry for &T {
    fn as_entry(&self) -> String {
        (*self).as_entry()
    }

    fn as_context(&self) -> &str {
        (*self).as_context()
    }
}

pub struct StatefulList<'items, T> {
    pub state: ListState,
    pub items: &'items RefCell<&'items mut Vec<T>>,
}

impl<'items, T: ListEntry> StatefulList<'items, T> {
    pub fn new(items: &'items RefCell<&'items mut Vec<T>>) -> StatefulList<'items, T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.borrow().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.borrow().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select(&mut self, selection: usize) {
        if selection < self.items.borrow().len() {
            self.state.select(Some(selection));
        } else {
            self.state.select(None);
        }
    }

    pub fn tui_list(&self) -> List<'items> {
        let items = self
            .items
            .borrow()
            .iter()
            .map(|item| {
                let span = Span::from((item).as_entry());
                ListItem::new(span)
            })
            .collect::<Vec<ListItem>>();

        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Your Flurry Generated Commands"),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
    }

    pub fn styled_tui_list(&self, style: Style, indices: &Vec<usize>) -> List<'items> {
        let items = self
            .items
            .borrow()
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let span = if indices.contains(&index) {
                    Span::styled(item.as_entry(), style)
                } else {
                    Span::from(item.as_entry())
                };
                ListItem::new(span)
            })
            .collect::<Vec<ListItem>>();

        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Your Flurry Generated Commands"),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
    }
}
