use crate::tui::{layout::*, widgets::*};

pub struct AddSequence {
    pub buf: String,
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

    pub fn hydrate(&mut self, add_cmd: &mut Option<AddCmdUi>) {
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

#[derive(Default)]
pub struct ScanDirOpts {
    pub depth: String,
    pub ext_filters: String,
    pub file_type_filter: String,
    pub raw_filters: String,
    pub regex_filters: String,
}

#[derive(Default)]
pub struct AddCmdUi {
    pub key: String,
    pub bin: String,
    pub joined_args: String,
    pub joined_aliases: String,
    pub encoder: String,
    pub permissions: String,
    pub query_which: String,
    pub scan_dir: Option<ScanDirOpts>,
}

fn parse_with_delim(arg: String, delimiter: &str) -> Option<Vec<String>> {
    let split: Vec<String> = arg
        .split(delimiter)
        .filter_map(|a| {
            if a.is_empty() {
                None
            } else {
                Some(a.to_owned())
            }
        })
        .collect();

    if split.is_empty() {
        None
    } else {
        Some(split)
    }
}

use crate::prelude::*;

impl AddCmdUi {
    pub fn to_cmd(self) -> Result<(String, GeneratedCommand)> {
        use crate::cli::types::{
            aliases_from_arg, args_from_arg, encoder_from_arg, exts_filter_from_arg,
            file_type_filter_from_arg, permissions_from_arg, recursion_limit_from_arg,
        };

        let aliases = aliases_from_arg(&self.joined_aliases).ok();
        let dfl_args = args_from_arg(&self.joined_args).ok();
        let encoder = encoder_from_arg(&self.encoder).ok();
        let permissions = permissions_from_arg(&self.permissions).or_else(|e| bail!(e))?;
        let query_which = match self.query_which.as_str() {
            "y" | "yes" | "true" => true,
            "f" | "n" | "no" | "false" => false,
            _ => return Err(anyhow!("not a valid input!")),
        };

        let (scan_dir, filter) = match self.scan_dir {
            None => (ScanDirKind::None, FiltersKind::None),
            Some(ScanDirOpts {
                depth,
                ext_filters,
                file_type_filter,
                raw_filters,
                regex_filters,
            }) => {
                let scan_dir = recursion_limit_from_arg(&depth).or_else(|e| bail!(e))?;
                let mut filters = vec![];

                if let Ok(ext_filters) = exts_filter_from_arg(&ext_filters) {
                    filters.push(ext_filters);
                }
                if let Ok(file_type_filter) = file_type_filter_from_arg(&file_type_filter) {
                    filters.push(file_type_filter);
                }
                if let Some(ref mut regex_filters) =
                    parse_with_delim(regex_filters, " ;;; ").map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::RegEx(f))
                            .collect::<Vec<FilterKind>>()
                    })
                {
                    filters.append(regex_filters);
                }
                if let Some(ref mut raw_filters) =
                    parse_with_delim(raw_filters, " ;;; ").map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::Raw(f))
                            .collect::<Vec<FilterKind>>()
                    })
                {
                    filters.append(raw_filters);
                }

                match filters.len() {
                    0 => (scan_dir, FiltersKind::None),
                    1 => (
                        scan_dir,
                        FiltersKind::One(filters.drain(..).nth(0).unwrap()),
                    ),
                    _ => (scan_dir, FiltersKind::Many(filters)),
                }
            }
        };

        Ok((
            self.key,
            GeneratedCommand {
                aliases,
                bin: self.bin,
                dfl_args,
                encoder,
                filter,
                permissions,
                query_which,
                scan_dir,
            },
        ))
    }
}
