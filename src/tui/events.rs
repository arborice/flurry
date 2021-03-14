use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind};

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
