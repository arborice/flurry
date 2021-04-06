use crate::tui::widgets::*;

pub fn rm_popup_label(selection: &Vec<String>) -> Paragraph {
    let rm_selection = selection.join(", ");
    let msg_span = Span::raw(format!("Delete {}?", rm_selection));
    let confirm_span = Span::raw("(y)es or (n)o?");

    Paragraph::new(Spans::from(vec![msg_span, confirm_span]))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}
