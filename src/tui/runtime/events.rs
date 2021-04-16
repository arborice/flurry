use crate::prelude::Seppuku;
use crossterm::event::{
    Event as CrossEvent, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use std::{
    sync::mpsc::{channel, Receiver},
    thread,
    time::{Duration, Instant},
};
use tinyvec::ArrayVec;

#[derive(Debug, PartialEq, Eq)]
pub struct Event(pub CrossEvent);

impl Event {
    pub const POLL_RATE: u64 = 100;

    pub fn spawn_loop(poll_rate: u64) -> Receiver<Event> {
        let (tx, rx) = channel();
        let tick_rate = Duration::from_millis(poll_rate);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if crossterm::event::poll(timeout).expect("Fatal Crossterm Poll Error") {
                    if let Ok(event) = crossterm::event::read() {
                        tx.send(Event(event)).seppuku(None);
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    last_tick = Instant::now();
                }
            }
        });
        rx
    }

    pub const CTRL_C: &'static str = "ctrl+c";
    pub const ESC: &'static str = "esc";
    pub const LEFT_CLICK: &'static str = "left";
    pub const RIGHT_CLICK: &'static str = "right";

    pub fn from_str(maybe_event: &str) -> Option<Self> {
        match maybe_event {
            Self::CTRL_C => Some(Self(CrossEvent::Key(KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char('c'),
            }))),
            Self::ESC => Some(Self(CrossEvent::Key(KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Esc,
            }))),
            Self::LEFT_CLICK => Some(Self(CrossEvent::Mouse(MouseEvent {
                modifiers: KeyModifiers::NONE,
                row: 0,
                column: 0,
                kind: MouseEventKind::Down(MouseButton::Left),
            }))),
            Self::RIGHT_CLICK => Some(Self(CrossEvent::Mouse(MouseEvent {
                modifiers: KeyModifiers::NONE,
                row: 0,
                column: 0,
                kind: MouseEventKind::Down(MouseButton::Right),
            }))),
            _ => None,
        }
    }

    pub fn is_next_trigger(&self) -> bool {
        if let CrossEvent::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            ..
        }) = self.0
        {
            true
        } else if let CrossEvent::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        }) = self.0
        {
            true
        } else {
            false
        }
    }

    pub fn is_prev_trigger(&self) -> bool {
        if let CrossEvent::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            ..
        }) = self.0
        {
            true
        } else if let CrossEvent::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        }) = self.0
        {
            true
        } else {
            false
        }
    }

    pub fn clicked_coords(&self) -> Option<(u16, u16)> {
        if let CrossEvent::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column,
            row,
            modifiers: KeyModifiers::NONE,
        }) = self.0
        {
            Some((column, row))
        } else {
            None
        }
    }
}

impl Into<Event> for char {
    fn into(self) -> Event {
        match self {
            '\n' => Event(CrossEvent::Key(KeyEvent::from(KeyCode::Enter))),
            _ => Event(CrossEvent::Key(KeyEvent::from(KeyCode::Char(self)))),
        }
    }
}

impl Default for Event {
    fn default() -> Event {
        Event(CrossEvent::Key(KeyEvent::from(KeyCode::Enter)))
    }
}

pub type Ec = [Event; 3];
pub type EventArray = ArrayVec<Ec>;

#[derive(Debug)]
pub struct EventHandler {
    pub accept: EventArray,
    pub reject: EventArray,
}

impl EventHandler {
    pub const ADD: char = 'a';
    pub const EDIT: char = 'e';
    pub const FILTER: char = 'f';
    pub const GO: char = 'g';
    pub const RM: char = 'r';

    pub fn accepts(&self, event: &Event) -> bool {
        self.accept.iter().any(|trigger| trigger == event)
    }

    pub fn rejects(&self, event: &Event) -> bool {
        self.reject.iter().any(|trigger| trigger == event)
    }
}

impl PartialEq<Event> for char {
    fn eq(&self, event: &Event) -> bool {
        let ev: Event = (*self).into();
        &ev == event
    }
}

impl PartialEq<char> for Event {
    fn eq(&self, ch: &char) -> bool {
        ch.eq(self)
    }
}

use crate::tui::{
    runtime::state::PopupState,
    widgets::{
        popup::{add::add_seq_items, edit::edit_seq_items, filters::filter_seq_items},
        UiStackSequence,
    },
};
use tinyvec::array_vec;

#[derive(Debug)]
pub struct StatefulEventHandler {
    pub state: PopupState,
    pub handler: EventHandler,
}

impl StatefulEventHandler {
    pub fn new() -> Self {
        Self {
            state: PopupState::Closed,
            handler: EventHandler {
                accept: array_vec!(Ec => 'q'.into(), ' '.into()),
                reject: array_vec!(Ec => Event::from_str(Event::CTRL_C).unwrap(), Event::from_str(Event::ESC).unwrap()),
            },
        }
    }

    pub fn for_add_popup() -> Self {
        Self {
            state: PopupState::Add(UiStackSequence::new(add_seq_items())),
            handler: EventHandler {
                accept: array_vec!(Ec => '\n'.into()),
                reject: array_vec!(Ec => Event::from_str(Event::ESC).unwrap(), Event::from_str(Event::CTRL_C).unwrap()),
            },
        }
    }

    pub fn for_edit_popup() -> Self {
        Self {
            state: PopupState::Edit(UiStackSequence::new(edit_seq_items())),
            handler: EventHandler {
                accept: array_vec!(Ec =>),
                reject: array_vec!(Ec =>),
            },
        }
    }

    pub fn for_filter_popup() -> Self {
        Self {
            state: PopupState::Filters(UiStackSequence::new(filter_seq_items())),
            handler: EventHandler {
                accept: array_vec!(Ec => '\n'.into()),
                reject: array_vec!(Ec => Event::from_str(Event::ESC).unwrap(), Event::from_str(Event::CTRL_C).unwrap()),
            },
        }
    }

    pub fn for_rm_popup() -> Self {
        Self {
            state: PopupState::RmConfirm,
            handler: EventHandler {
                accept: array_vec!(Ec => 'y'.into()),
                reject: array_vec!(Ec => 'n'.into(), Event::from_str(Event::CTRL_C).unwrap(), Event::from_str(Event::ESC).unwrap()),
            },
        }
    }
}

pub fn event_handlers() -> [StatefulEventHandler; 7] {
    [
        StatefulEventHandler::new(),
        StatefulEventHandler::for_add_popup(),
        StatefulEventHandler::for_edit_popup(),
        StatefulEventHandler::for_filter_popup(),
        StatefulEventHandler::for_rm_popup(),
        StatefulEventHandler {
            state: PopupState::Info,
            handler: EventHandler {
                accept: array_vec!(Ec =>),
                reject: array_vec!(Ec =>),
            },
        },
        StatefulEventHandler {
            state: PopupState::ExitWithMsg,
            handler: EventHandler {
                accept: array_vec!(Ec =>),
                reject: array_vec!(Ec =>),
            },
        },
    ]
}
