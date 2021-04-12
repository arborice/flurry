use crate::tui::{layout::*, widgets::*};

#[derive(Debug)]
pub enum PopupState {
    Add(UiStackSequence<{ popup::add::ADD_SEQ_NUM_ITEMS }>),
    Closed,
    Edit(UiStackSequence<{ popup::edit::EDIT_SEQ_NUM_FRAMES }>),
    ExitWithMsg,
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
            ExitWithMsg => "ExitWithMsg",
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
            ExitWithMsg => match other {
                ExitWithMsg => true,
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
                if let Some(ref err) = seq.err_msg {
                    let err_rect = centered_rect(75, 1, frame.size());
                    let err_label = popup::info::info_label(err, true);
                    frame.render_widget(err_label, err_rect);
                }
            }
            PopupState::Edit(seq) => {
                let seq_frame = seq.current_frame();
                let query = seq_frame.query;
                let height = if seq.err_msg.is_none() { 5 } else { 6 };

                let popup_rect = centered_rect(75, height, frame.size());
                let popup_block = Block::default().title(query).borders(Borders::ALL);

                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                popup::add::centered_input_block(frame, seq.buf.as_str());
                if let Some(ref err) = seq.err_msg {
                    let err_rect = centered_rect(75, 1, frame.size());
                    let err_label = popup::info::info_label(err, true);
                    frame.render_widget(err_label, err_rect);
                }
                todo!("")
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
            PopupState::Info | PopupState::ExitWithMsg => {
                let frame_size = frame.size();
                let popup_rect = centered_rect(60, 5, frame_size);
                let popup_block = Block::default().title(self.as_ref()).borders(Borders::ALL);

                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                let popup_label = popup::info::info_label(context.as_ref().unwrap(), false);
                let label_rect =
                    centered_rect_with_margin(60, 5, frame_size, (Direction::Vertical, 1));
                frame.render_widget(popup_label, label_rect);
            }
            _ => {}
        }
    }
}
