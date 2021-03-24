use super::runtime::Callback;
use crate::prelude::*;
use std::{
    sync::mpsc::{channel, Receiver},
    thread,
    time::{Duration, Instant},
};

const EVENT_POLL_RATE: u64 = 100;

pub fn spawn_event_loop() -> Receiver<Event> {
    let (tx, rx) = channel();
    let tick_rate = Duration::from_millis(EVENT_POLL_RATE);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout).unwrap() {
                if let Ok(event) = crossterm::event::read() {
                    tx.send(event).seppuku(None);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    });
    rx
}

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind};
use tinyvec::{array_vec, ArrayVec};

pub type Ec = [EventCap; 3];
pub type EventArray = ArrayVec<Ec>;

pub struct TuiInputHandler {
    pub accept: EventArray,
    pub exit: EventArray,
    pub reject: EventArray,
    pub select: EventArray,
    pub unselect: EventArray,
}

impl Default for TuiInputHandler {
    fn default() -> TuiInputHandler {
        TuiInputHandler {
            accept: array_vec!(Ec => EventCap::with_key('y')),
            exit: array_vec!(Ec => EventCap::with_key('q'), EventCap::ctrl_c()),
            reject: array_vec!(Ec => EventCap::with_key('n'), EventCap::Key(KeyEvent::from(KeyCode::Esc))),
            select: array_vec!(Ec => EventCap::with_key(' '), EventCap::with_key('\n'), EventCap::LeftClick),
            unselect: array_vec!(Ec => EventCap::with_key('u'), EventCap::with_key('r'), EventCap::Key(KeyEvent::from(KeyCode::Delete))),
        }
    }
}

impl TuiInputHandler {
	pub const ADD: char = 'a';
	pub const GO: char = 'g';
	pub const RM: char = 'r';

    pub fn trigger_add(&self, ev: &Event) -> bool {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(Self::ADD),
            modifiers: KeyModifiers::NONE,
        }) = ev
        {
            true
        } else {
            false
        }
    }
    
    pub fn trigger_go(&self, ev: &Event) -> bool {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(Self::GO),
            modifiers: KeyModifiers::NONE,
        }) = ev
        {
            true
        } else {
            false
        }
    }

    pub fn trigger_rm(&self, ev: &Event) -> bool {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(Self::RM),
            modifiers: KeyModifiers::NONE,
        }) = ev
        {
            true
        } else {
            false
        }
    }

    pub fn accepts(&self, ev: &Event) -> bool {
        self.accept.iter().any(|input| input == ev)
    }

    pub fn rejects(&self, ev: &Event) -> bool {
        self.reject.iter().any(|input| input == ev)
    }

    pub fn selects(&self, ev: &Event) -> bool {
        self.select.iter().any(|input| input == ev)
    }

    pub fn unselects(&self, ev: &Event) -> bool {
        self.unselect.iter().any(|input| input == ev)
    }

    pub fn is_exit_trigger(&self, ev: &Event) -> bool {
        self.exit.iter().any(|input| input == ev)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum EventCap {
    Key(KeyEvent),
    LeftClick,
    #[allow(dead_code)]
    RightClick,
}

impl Default for EventCap {
    fn default() -> EventCap {
        EventCap::Key(KeyEvent::from(KeyCode::Null))
    }
}

impl EventCap {
    pub fn with_key(key: char) -> Self {
        Self::Key(KeyEvent::from(KeyCode::Char(key)))
    }

    pub fn ctrl_c() -> Self {
        Self::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
    }
}

impl PartialEq<Event> for EventCap {
    fn eq(&self, event: &Event) -> bool {
        match self {
            EventCap::Key(key) => {
                if let Event::Key(ke) = event {
                    ke == key
                } else {
                    false
                }
            }
            EventCap::LeftClick => {
                if let Event::Mouse(me) = event {
                    me.kind == MouseEventKind::Down(MouseButton::Left)
                } else {
                    false
                }
            }
            EventCap::RightClick => {
                if let Event::Mouse(me) = event {
                    me.kind == MouseEventKind::Down(MouseButton::Right)
                } else {
                    false
                }
            }
        }
    }
}
