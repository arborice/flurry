pub mod events;
pub mod state;
pub mod table;

use events::*;
use tinyvec::array_vec;

pub struct StatefulEventHandler<'s> {
    pub state: state::PopupState<'s>,
    pub handler: events::EventHandler,
}

impl StatefulEventHandler<'_> {
    pub fn new() -> Self {
        Self {
            state: state::PopupState::Closed,
            handler: EventHandler {
                accept: array_vec!(Ec => '\n'.into(), ' '.into(), Event::from_str(Event::LEFT_CLICK).unwrap()),
                reject: array_vec!(Ec => 'q'.into(), Event::from_str(Event::CTRL_C).unwrap(), Event::from_str(Event::ESC).unwrap()),
            },
        }
    }
}
