use crate::{
    prelude::*,
    tui::{
        events::{Event, EventCap, Events},
        widgets::popup::PopupOpts,
    },
};
use termion::{
    event::Key,
    input::MouseTerminal,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tinyvec::{array_vec, ArrayVec};
use tui::{backend::TermionBackend, style::Style, Terminal};

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
            reject: array_vec!(Ec => EventCap::with_key('n'), EventCap::Key(Key::Esc)),
            select: array_vec!(Ec => EventCap::with_key(' '), EventCap::with_key('\n'), EventCap::LeftClick),
            unselect: array_vec!(Ec => EventCap::with_key('u'), EventCap::with_key('r'), EventCap::Key(Key::Delete)),
        }
    }
}

impl TuiInputHandler {
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

pub enum TuiCallback<F: FnMut(usize)> {
    Halting(F),
    NonHalting(F),
}

type FlurryTerminal =
    Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<std::io::Stdout>>>>>;

pub struct TuiOpts<'opts, F: FnMut(usize)> {
    pub events: Events,
    pub terminal: FlurryTerminal,
    pub popup_options: Option<&'opts PopupOpts<'opts>>,
    pub callback: TuiCallback<F>,
    pub selected_style: Option<Style>,
    pub input_handler: TuiInputHandler,
}

impl<'opts, F: FnMut(usize)> TuiOpts<'opts, F> {
    pub fn new(
        input_handler: TuiInputHandler,
        events: Events,
        callback: TuiCallback<F>,
    ) -> Result<TuiOpts<'opts, F>> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let mouse_term = MouseTerminal::from(stdout);
        let alt_screen = AlternateScreen::from(mouse_term);
        let termionated = TermionBackend::new(alt_screen);
        let terminal = Terminal::new(termionated)?;

        Ok(Self {
            events,
            terminal,
            popup_options: None,
            selected_style: None,
            callback,
            input_handler,
        })
    }

    pub fn with_popup(mut self, opts: &'opts PopupOpts<'opts>) -> Self {
        self.popup_options = Some(opts);
        self
    }

    pub fn with_selection_highlighter(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }
}
