pub mod io;
pub mod layouts;
pub mod runtime;
pub mod widgets;

pub mod prelude {
    pub use super::{
        io::*,
        runtime::list::render,
        widgets::{list::ListEntry, popup::PopupOpts},
        TuiCallback, TuiOpts,
    };
    pub use std::cell::RefCell;
    pub use tinyvec::array_vec;
    pub use tui::style::{Color, Modifier, Style};
}

use io::TuiInputHandler;
use widgets::popup::PopupOpts;
pub enum TuiCallback<F: FnMut(usize)> {
    Halting(F),
    NonHalting(F),
}

pub struct TuiOpts<'opts, F: FnMut(usize)> {
    pub popup_options: Option<&'opts PopupOpts<'opts>>,
    pub callback: TuiCallback<F>,
    pub selected_style: Option<tui::style::Style>,
    pub input_handler: TuiInputHandler,
}

impl<'opts, F: FnMut(usize)> TuiOpts<'opts, F> {
    pub fn new(input_handler: TuiInputHandler, callback: TuiCallback<F>) -> TuiOpts<'opts, F> {
        Self {
            popup_options: None,
            selected_style: None,
            callback,
            input_handler,
        }
    }

    pub fn with_popup(mut self, opts: &'opts PopupOpts<'opts>) -> Self {
        self.popup_options = Some(opts);
        self
    }

    pub fn with_selection_highlighter(mut self, style: tui::style::Style) -> Self {
        self.selected_style = Some(style);
        self
    }
}
