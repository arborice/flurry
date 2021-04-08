pub mod popup;
pub mod table;

pub use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub struct SeqFrame {
    pub query: &'static str,
    pub buf: String,
    pub validator: fn(&str) -> bool,
    pub err_msg: &'static str,
}

use std::fmt;
impl fmt::Debug for SeqFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SeqFrame")
            .field("query", &self.query)
            .field("buf", &self.buf)
            .field("err_msg", &self.err_msg)
            .finish()
    }
}

impl SeqFrame {
    pub fn new(query: &'static str, err_msg: &'static str, validator: fn(&str) -> bool) -> Self {
        Self {
            buf: String::new(),
            query,
            err_msg,
            validator,
        }
    }
}

use crate::prelude::*;

#[derive(Debug)]
pub struct UiStackSequence<const NUM_FRAMES: usize> {
    pub buf: String,
    pub err_msg: Option<String>,
    stages: [SeqFrame; NUM_FRAMES],
    index: usize,
}

impl<const NUM_FRAMES: usize> UiStackSequence<NUM_FRAMES> {
    pub fn new(stages: [SeqFrame; NUM_FRAMES]) -> Self {
        Self {
            buf: String::new(),
            err_msg: None,
            index: 0,
            stages,
        }
    }

    pub fn current_frame(&self) -> &SeqFrame {
        &self.stages[self.index]
    }

    pub fn done(&self) -> bool {
        self.index == self.stages.len()
    }

    pub fn try_push(&mut self) -> Result<()> {
        if self.index >= NUM_FRAMES {
            return Err(anyhow!(
                "{:#?} has length of {}. Attempted out of range access.",
                self,
                NUM_FRAMES
            ));
        }

        let SeqFrame {
            ref mut buf,
            ref validator,
            ref err_msg,
            ..
        } = self.stages[self.index];

        if validator(&self.buf) {
            *buf = self.buf.drain(..).collect();
            self.index += 1;
        } else {
            self.err_msg.replace(err_msg.to_string());
        }
        Ok(())
    }

    pub fn pop(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.buf.clone_from(&self.stages[self.index].buf);
        }
    }

    pub fn print(&mut self, input: char) {
        self.buf.push(input);
    }

    pub fn delete(&mut self) {
        self.buf.pop();
    }
}
