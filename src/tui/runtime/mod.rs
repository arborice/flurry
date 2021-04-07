pub mod events;
pub mod state;
pub mod table;

use events::*;
use tinyvec::array_vec;

pub fn event_handlers() -> [StatefulEventHandler; 5] {
    [
        StatefulEventHandler::new(),
        StatefulEventHandler::for_add_popup(),
        StatefulEventHandler::for_edit_popup(),
        StatefulEventHandler::for_rm_popup(),
        StatefulEventHandler {
            state: state::PopupState::ExitError,
            handler: EventHandler {
                accept: array_vec!(Ec =>),
                reject: array_vec!(Ec =>),
            },
        },
    ]
}

#[derive(Debug)]
pub struct StatefulEventHandler {
    pub state: state::PopupState,
    pub handler: events::EventHandler,
}

impl StatefulEventHandler {
    pub fn new() -> Self {
        Self {
            state: state::PopupState::Closed,
            handler: EventHandler {
                accept: array_vec!(Ec => '\n'.into(), ' '.into(), Event::from_str(Event::LEFT_CLICK).unwrap()),
                reject: array_vec!(Ec => 'q'.into(), Event::from_str(Event::CTRL_C).unwrap(), Event::from_str(Event::ESC).unwrap()),
            },
        }
    }

    pub fn for_add_popup() -> Self {
        Self {
            state: state::PopupState::Add(super::widgets::popup::add::AddSequence::new()),
            handler: EventHandler {
                accept: array_vec!(Ec => '\n'.into()),
                reject: array_vec!(Ec => Event::from_str("esc").unwrap(), Event::from_str("ctrl+c").unwrap()),
            },
        }
    }

    pub fn for_edit_popup() -> Self {
        Self {
            state: state::PopupState::Edit,
            handler: EventHandler {
                accept: array_vec!(Ec =>),
                reject: array_vec!(Ec =>),
            },
        }
    }

    pub fn for_rm_popup() -> Self {
        Self {
            state: state::PopupState::RmConfirm,
            handler: EventHandler {
                accept: array_vec!(Ec => 'y'.into()),
                reject: array_vec!(Ec => 'n'.into(), Event::from_str("ctrl+c").unwrap(), Event::from_str("esc").unwrap()),
            },
        }
    }
}
