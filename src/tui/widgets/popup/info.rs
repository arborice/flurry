use crate::tui::widgets::*;

pub fn info_label<'r, S: ToString>(message: S) -> Paragraph<'r> {
    let msg_span = Spans::from(message.to_string());

    Paragraph::new(msg_span)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
}
