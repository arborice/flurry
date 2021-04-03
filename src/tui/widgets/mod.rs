pub mod edit_popup;
pub mod table;

use super::{centered_rect, centered_rect_with_margin};

use tui::{
    backend::Backend,
    layout::{Alignment, Direction},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub enum PopupWidget<'c> {
    Add {
        message: &'c str,
        title: &'c str,
    },
    Confirm {
        message: &'c str,
        title: &'c str,
        context: Option<&'c str>,
    },
    Hidden,
}

impl<'c> Default for PopupWidget<'c> {
    fn default() -> PopupWidget<'c> {
        PopupWidget::Hidden
    }
}

impl<'c> PopupWidget<'c> {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        match self {
            PopupWidget::Add { message, title } => {
                let popup_rect = centered_rect(75, 5, frame.size());
                let popup_block = Block::default().title(*title).borders(Borders::ALL);
                let input_label = centered_input_label(message);
                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);
                frame.render_widget(input_label, popup_rect);
                centered_input_block(frame);
            }
            PopupWidget::Confirm {
                message,
                title,
                context,
            } => {
                let frame_size = frame.size();
                let popup_rect = centered_rect(60, 4, frame_size);
                let popup_block = Block::default().title(*title).borders(Borders::ALL);
                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                let popup_label = centered_label(message, context);
                let label_rect =
                    centered_rect_with_margin(60, 4, frame_size, (Direction::Vertical, 1));
                frame.render_widget(popup_label, label_rect);
            }
            _ => {}
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            PopupWidget::Hidden => false,
            _ => true,
        }
    }

    pub fn closed() -> Self {
        Self::default()
    }
}

fn centered_input_block<B: Backend>(frame: &mut Frame<B>) {
    let input_rect = centered_rect_with_margin(65, 1, frame.size(), (Direction::Vertical, 1));
    let input_block = Block::default().borders(Borders::BOTTOM);
    frame.render_widget(input_block, input_rect);
}

fn centered_input_label(input_label: &str) -> Paragraph {
    Paragraph::new(Spans::from(input_label))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}

fn centered_label<'c>(message: &'c str, context: &'c Option<&'c str>) -> Paragraph<'c> {
    let msg_span = match context {
        Some(ctx) => Span::raw(message.replace("{{ ctx }}", ctx)),
        None => Span::raw(message),
    };
    let confirm_span = Span::raw("(y)es or (n)o?");
    Paragraph::new(Spans::from(vec![msg_span, confirm_span]))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
