use std::{
    io::stdin,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
};
use termion::{
    event::{Event as TermEv, Key, MouseButton, MouseEvent},
    input::TermRead,
};

pub enum Event {
    KeyPress(Key),
    Mouse(MouseEvent),
}

pub struct Events(mpsc::Receiver<Event>);

use crate::tui::opts::EventArray;
impl Events {
    pub fn with_exit_triggers(exit_triggers: &EventArray) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let exit_triggers = exit_triggers.clone();

        thread::spawn(move || {
            for evt in stdin().events() {
                if let Ok(ev) = evt {
                    match ev {
                        TermEv::Key(key) => {
                            if let Err(err) = tx.send(Event::KeyPress(key)) {
                                eprintln!("{}", err);
                                return;
                            }
                            if !ignore_exit_key.load(Ordering::Relaxed)
                                && exit_triggers.iter().any(|t| t == &Event::KeyPress(key))
                            {
                                return;
                            }
                        }
                        TermEv::Mouse(me) => {
                            if let Err(err) = tx.send(Event::Mouse(me)) {
                                eprintln!("{}", err);
                                return;
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
        Events(rx)
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.0.recv()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum EventCap {
    Key(Key),
    LeftClick,
    #[allow(dead_code)]
    RightClick,
}

impl Default for EventCap {
    fn default() -> EventCap {
        EventCap::Key(Key::Null)
    }
}

impl EventCap {
    pub fn with_key(key: char) -> Self {
        Self::Key(Key::Char(key))
    }

    pub fn ctrl_c() -> Self {
        Self::Key(Key::Ctrl('c'))
    }
}

impl PartialEq<Event> for EventCap {
    fn eq(&self, event: &Event) -> bool {
        match self {
            EventCap::LeftClick => {
                if let Event::Mouse(MouseEvent::Press(MouseButton::Left, ..)) = event {
                    true
                } else {
                    false
                }
            }
            EventCap::RightClick => {
                if let Event::Mouse(MouseEvent::Press(MouseButton::Right, ..)) = event {
                    true
                } else {
                    false
                }
            }
            EventCap::Key(k) => {
                if let Event::KeyPress(key) = event {
                    k == key
                } else {
                    false
                }
            }
        }
    }
}
