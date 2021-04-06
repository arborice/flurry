use crate::tui::widgets::*;

pub fn rm_popup_label<'r, S: std::fmt::Display>(selection: S) -> Paragraph<'r> {
    let msg_span = Span::raw(format!("Delete {}?", selection));
    let confirm_span = Span::raw("(y)es or (n)o?");

    Paragraph::new(Spans::from(vec![msg_span, confirm_span]))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}
