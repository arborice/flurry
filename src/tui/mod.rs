// pub mod io;
pub mod layout;
pub mod runtime;
pub mod widgets;

pub mod prelude {
    pub use super::{
        runtime::{events::*, state::*},
        widgets::table::StatefulCmdsTable,
    };
    pub use std::cell::RefCell;
    pub use tui::style::{Color, Modifier, Style};
}
