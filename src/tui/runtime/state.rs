use crate::tui::{layout::*, widgets::*};

#[derive(Debug)]
pub enum PopupState {
    Add(UiStackSequence<7>),
    Closed,
    Edit(UiStackSequence<8>),
    ExitError,
    Info,
    RmConfirm,
}

impl AsRef<str> for PopupState {
    fn as_ref(&self) -> &str {
        use PopupState::*;
        match self {
            Add(_) => "Add",
            Closed => "Closed",
            Edit(_) => "Edit",
            ExitError => "ExitError",
            Info => "Info",
            RmConfirm => "Confirm Removal",
        }
    }
}

impl PartialEq<PopupState> for PopupState {
    fn eq(&self, other: &PopupState) -> bool {
        use PopupState::*;
        match self {
            Add(_) => match other {
                Add(_) => true,
                _ => false,
            },
            Edit(_) => match other {
                Edit(_) => true,
                _ => false,
            },
            ExitError => match other {
                ExitError => true,
                _ => false,
            },
            Info => match other {
                Info => true,
                _ => false,
            },
            RmConfirm => match other {
                RmConfirm => true,
                _ => false,
            },
            Closed => match other {
                Closed => true,
                _ => false,
            },
        }
    }
}

impl PartialEq<PopupState> for &PopupState {
    fn eq(&self, other: &PopupState) -> bool {
        (*self).eq(other)
    }
}

impl PopupState {
    pub fn is_open(&self) -> bool {
        self == PopupState::Closed
    }

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, context: &Option<String>) {
        match self {
            PopupState::Add(seq) => {
                let seq_frame = seq.current_frame();
                let query = seq_frame.query;
                let height = if seq.err_msg.is_none() { 5 } else { 6 };

                let popup_rect = centered_rect(75, height, frame.size());
                let popup_block = Block::default().title(query).borders(Borders::ALL);

                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                popup::add::centered_input_block(frame, seq.buf.as_str());
            }
            PopupState::RmConfirm => {
                let frame_size = frame.size();
                let popup_rect = centered_rect(60, 4, frame_size);
                let popup_block = Block::default().title(self.as_ref()).borders(Borders::ALL);

                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                let popup_label = popup::rm::rm_popup_label(context.as_ref().unwrap());
                let label_rect =
                    centered_rect_with_margin(60, 4, frame_size, (Direction::Vertical, 1));
                frame.render_widget(popup_label, label_rect);
            }
            PopupState::Info | PopupState::ExitError => {
                let frame_size = frame.size();
                let popup_rect = centered_rect(60, 5, frame_size);
                let popup_block = Block::default().title(self.as_ref()).borders(Borders::ALL);

                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                let popup_label = popup::info::info_label(context.as_ref().unwrap());
                let label_rect =
                    centered_rect_with_margin(60, 5, frame_size, (Direction::Vertical, 1));
                frame.render_widget(popup_label, label_rect);
            }
            _ => {}
        }
    }
}
