use crate::tui::{layout::*, widgets::*};

pub enum PopupState {
    Add(popup::add::AddSequence),
    Edit,
    RmConfirm,
    Closed,
}

impl PartialEq<PopupState> for PopupState {
    fn eq(&self, other: &PopupState) -> bool {
        use PopupState::*;
        match self {
            Add(_) => match other {
                Add(_) => true,
                _ => false,
            },
            Edit => match other {
                Edit => true,
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

    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        let frame_size = frame.size();
        match self {
            PopupState::Add(seq) => {
                let query = seq.current_frame();

                let popup_rect = centered_rect(75, 5, frame_size);
                let popup_block = Block::default().title(query).borders(Borders::ALL);

                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                popup::add::centered_input_block(frame, seq.buf.as_str());
            }
            PopupState::Confirm {
                message,
                title,
                context,
            } => {
                let popup_rect = centered_rect(60, 4, frame_size);
                let popup_block = Block::default().title(*title).borders(Borders::ALL);
                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                let popup_label = popup::rm::centered_label(message, context);
                let label_rect =
                    centered_rect_with_margin(60, 4, frame_size, (Direction::Vertical, 1));
                frame.render_widget(popup_label, label_rect);
            }
            _ => {}
        }
    }
}
