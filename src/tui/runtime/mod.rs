pub mod table;

use super::io::TuiInputHandler;

#[derive(Default)]
pub struct TuiOpts {
    pub input_handler: TuiInputHandler,
    pub last_requested_callback: Option<char>,
    pub selected_style: tui::style::Style,
}

impl TuiOpts {
    pub fn with_input_handler(mut self, input_handler: TuiInputHandler) -> Self {
        self.input_handler = input_handler;
        self
    }

    pub fn with_selection_highlighter(mut self, style: tui::style::Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn request_add(&mut self) {
        self.last_requested_callback.replace(TuiInputHandler::ADD);
    }

    pub fn request_go(&mut self) {
        self.last_requested_callback.replace(TuiInputHandler::GO);
    }

    pub fn request_rm(&mut self) {
        self.last_requested_callback.replace(TuiInputHandler::RM);
    }
}
