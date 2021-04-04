pub mod edit_popup;
pub mod table;

use super::{centered_rect, centered_rect_with_margin, runtime::table::AddCmdUi};

use tui::{
    backend::Backend,
    layout::{Alignment, Direction},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub struct AddSequence {
    buf: String,
    stages: [(&'static str, String); 7],
    index: usize,
}

impl AddSequence {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            stages: [
                ("Key (trigger) for new command?", String::new()),
                ("Binary/primary command?", String::new()),
                ("Arguments for command?", String::new()),
                ("Aliases for command?", String::new()),
                ("Encoder for output?", String::new()),
                ("Permissions schema?", String::new()),
                ("Query which?", String::new()),
            ],
            index: 0,
        }
    }

    fn current_frame(&self) -> &str {
        &self.stages[self.index].0
    }

    pub fn done(&self) -> bool {
        self.index == self.stages.len()
    }

    pub fn populate(&mut self, add_cmd: &mut Option<AddCmdUi>) {
        add_cmd.replace(AddCmdUi {
            key: self.stages[0].1.drain(..).collect(),
            bin: self.stages[1].1.drain(..).collect(),
            joined_args: self.stages[2].1.drain(..).collect(),
            joined_aliases: self.stages[3].1.drain(..).collect(),
            encoder: self.stages[4].1.drain(..).collect(),
            permissions: self.stages[5].1.drain(..).collect(),
            query_which: self.stages[6].1.drain(..).collect(),
            scan_dir: None,
        });
    }

    pub fn push(&mut self) {
        self.stages[self.index].1 = self.buf.clone();

        if self.index < self.stages.len() {
            self.index += 1;
        }

        self.buf.clear();
    }

    pub fn pop(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.buf.clone_from(&self.stages[self.index].1);
        }
    }

    pub fn print(&mut self, input: char) {
        self.buf.push(input);
    }

    pub fn delete(&mut self) {
        self.buf.pop();
    }
}

pub enum PopupWidget<'c> {
    Add(AddSequence),
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
            PopupWidget::Add(seq) => {
                let query = seq.current_frame();

                let popup_rect = centered_rect(75, 5, frame.size());
                let popup_block = Block::default().title(query).borders(Borders::ALL);

                frame.render_widget(Clear, popup_rect);
                frame.render_widget(popup_block, popup_rect);

                centered_input_block(frame, seq.buf.as_str());
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

fn centered_input_block<B: Backend>(frame: &mut Frame<B>, content: &str) {
    let input_rect = centered_rect_with_margin(65, 1, frame.size(), (Direction::Vertical, 1));
    let input_block = Block::default().borders(Borders::BOTTOM);
    frame.render_widget(input_block, input_rect);

    let disp_rect = centered_rect(65, 1, frame.size());
    let display = Paragraph::new(Spans::from(content)).wrap(Wrap { trim: false });
    frame.render_widget(display, disp_rect);
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
