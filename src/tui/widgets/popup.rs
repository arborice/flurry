use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub struct PopupOpts<'pop> {
    pub message: &'pop str,
    pub title: &'pop str,
}

pub fn centered_rect(percent_x: u16, popup_height: u16, frame_size: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((frame_size.height - popup_height) / 2),
                Constraint::Length(popup_height),
                Constraint::Length((frame_size.height - popup_height) / 2),
            ]
            .as_ref(),
        )
        .split(frame_size);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

use crate::{
    prelude::*,
    tui::widgets::list::{ListEntry, StatefulList},
};

pub fn centered_label<Ctx: ListEntry>(message: &str, context: Option<Ctx>) -> Paragraph {
    let msg_span = match context {
        Some(ctx) => Spans::from(message.replace("{{ ctx }}", ctx.as_context())),
        None => Spans::from(message),
    };
    let confirm_span = Spans::from("(y)es or (n)o?");
    Paragraph::new(vec![Spans::from(""), msg_span, confirm_span])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn render_popup<T: ListEntry, B: Backend>(
    app: &StatefulList<T>,
    frame: &mut Frame<B>,
    opts: &PopupOpts,
) {
    let frame_size = frame.size();
    let popup_label = centered_label(
        opts.message,
        Some(&app.items.borrow()[app.state.selected().sudoku("runtime failure :(")]),
    );
    let popup_rect = centered_rect(60, 4, frame_size);
    let popup_block = Block::default().title(opts.title).borders(Borders::ALL);
    frame.render_widget(Clear, popup_rect);
    frame.render_widget(popup_block, popup_rect);
    frame.render_widget(popup_label, popup_rect);
}
