use crate::tui::{layout::*, runtime::table::AddCmdUi, widgets::*};

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

    pub fn current_frame(&self) -> &str {
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

pub fn centered_input_block<B: Backend>(frame: &mut Frame<B>, content: &str) {
    let input_rect = centered_rect_with_margin(65, 1, frame.size(), (Direction::Vertical, 1));
    let input_block = Block::default().borders(Borders::BOTTOM);
    frame.render_widget(input_block, input_rect);

    let disp_rect = centered_rect(65, 1, frame.size());
    let display = Paragraph::new(Spans::from(content)).wrap(Wrap { trim: false });
    frame.render_widget(display, disp_rect);
}
