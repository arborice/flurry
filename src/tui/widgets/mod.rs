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
            .field("validator", &"VALIDATOR FUNCTION")
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

pub struct UiStack;
impl UiStack {
    pub const KEY: &'static str = "Key (trigger) for new command?";
    pub const BIN: &'static str = "Binary/primary command?";
    pub const ARGS: &'static str = "Arguments for command?";
    pub const ALIASES: &'static str = "Aliases for command?";
    pub const ENCODER: &'static str = "Encoder for output?";
    pub const PERMISSIONS: &'static str = "Permissions schema?";
    pub const SCAN_DIR: &'static str =
        "Scan directory? (Enter an integer to set fixed recursion limit)";
    pub const WHICH: &'static str = "Query which?";

    pub const KEY_ERR: &'static str = "key cannot be empty";
    pub const BIN_ERR: &'static str = "trigger cannot be empty";
    pub const ARGS_ERR: &'static str = "";
    pub const ALIASES_ERR: &'static str = "";
    pub const ENCODER_ERR: &'static str = "valid values: none, json, url";
    pub const PERMISSIONS_ERR: &'static str = "Permissions schema?";
    pub const SCAN_DIR_ERR: &'static str = "valid values: max, recursive, none, {int} (max 255)";
    pub const WHICH_ERR: &'static str = "(y)es or (n)o";
}

use crate::prelude::{anyhow, Result};

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
