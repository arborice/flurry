use crate::tui::widgets::*;

pub fn centered_label<'c>(message: &'c str, context: &'c Option<&'c str>) -> Paragraph<'c> {
    let msg_span = match context {
        Some(ctx) => Span::raw(message.replace("{{ ctx }}", ctx)),
        None => Span::raw(message),
    };
    let confirm_span = Span::raw("(y)es or (n)o?");
    Paragraph::new(Spans::from(vec![msg_span, confirm_span]))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
