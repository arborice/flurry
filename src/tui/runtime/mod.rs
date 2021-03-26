pub mod table;

use super::io::TuiInputHandler;

#[derive(Default)]
pub struct TuiOpts {
    pub input_handler: TuiInputHandler,
    pub selected_style: tui::style::Style,
}

impl TuiOpts {
    #[allow(dead_code)]
    pub fn with_input_handler(mut self, input_handler: TuiInputHandler) -> Self {
        self.input_handler = input_handler;
        self
    }

    pub fn with_selection_highlighter(mut self, style: tui::style::Style) -> Self {
        self.selected_style = style;
        self
    }
}
