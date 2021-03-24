pub mod io;
pub mod layouts;
pub mod runtime;
pub mod widgets;

pub mod prelude {
    pub use super::{
        io::*,
        runtime::TuiOpts,
        widgets::{popup::PopupOpts, table::StatefulCmdsTable},
    };
    pub use std::cell::RefCell;
    pub use tui::style::{Color, Modifier, Style};
}
