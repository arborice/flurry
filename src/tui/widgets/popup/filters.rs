use crate::{prelude::*, tui::widgets::*, utils::parse::*};

pub const FILTER_SEQ_NUM_FRAMES: usize = 4;

#[derive(Default)]
pub struct FilterContainer {
    filters: Vec<FilterKind>,
    pub ready: bool,
}

impl FilterContainer {
    pub fn clear(&mut self) {
        self.filters.clear();
        self.ready = false;
    }

    pub fn clone_from_cmd(&mut self, cmd: &GeneratedCommand) {
        match &cmd.filter {
            FiltersKind::None => {
                self.filters.clear();
                self.filters.reserve_exact(3);
            }
            FiltersKind::One(f) => {
                self.filters.truncate(1);
                self.filters.reserve_exact(3);
                self.filters[0] = f.clone();
            }
            FiltersKind::Many(filters) => {
                self.filters = filters.clone();
                self.filters.reserve_exact(3);
            }
        }
        self.ready = true;
    }

    pub fn drain_into_cmd(&mut self, cmd: &mut GeneratedCommand) {
        cmd.filter = match self.filters.len() {
            0 => FiltersKind::None,
            1 => FiltersKind::One(self.filters.remove(0)),
            _ => FiltersKind::Many(self.filters.drain(..).collect()),
        };
        self.ready = false;
    }

    pub fn hydrate_bufs(&self, buf_stack: &mut UiStackSequence<{ FILTER_SEQ_NUM_FRAMES }>) {
        let mut raw_filters = vec![];
        let mut regex_filters = vec![];
        self.filters.iter().for_each(|f| match f {
            FilterKind::Exts(exts) => buf_stack.stages[0].buf.clone_from(&exts.join(", ")),
            FilterKind::FileType(ft) => buf_stack.stages[1].buf.push_str(ft.as_ref()),
            FilterKind::Raw(raw) => raw_filters.push(raw.clone()),
            FilterKind::RegEx(regex) => regex_filters.push(regex.clone()),
            FilterKind::None => {}
        });

        buf_stack.stages[2].buf = raw_filters.join(" ; ");
        buf_stack.stages[3].buf = raw_filters.join(" ; ");
    }
}

pub struct FilterSeq;
impl FilterSeq {
    pub const EXTS: &'static str = "0";
    pub const FILE_TYPE: &'static str = "1";
    pub const RAW: &'static str = "2";
    pub const REGEX: &'static str = "3";

    pub const EXTS_ERR: &'static str = "0";
    pub const FILE_TYPE_ERR: &'static str = "1";
    pub const RAW_ERR: &'static str = "2";
    pub const REGEX_ERR: &'static str = "3";

    pub fn set_new_val(
        key: &str,
        new_val: &String,
        container: &mut FilterContainer,
    ) -> Result<(), String> {
        match key {
            Self::EXTS => {
                let exts_filter = exts_filter_from_arg(new_val.as_str())?;
                container.filters.push(exts_filter);
            }
            Self::FILE_TYPE => {
                let file_type_filter = file_type_filter_from_arg(new_val.as_str())?;
                container.filters.push(file_type_filter);
            }
            Self::RAW => {
                if let Some(mut raw_filters) =
                    parse_with_delim(new_val.as_str(), " ;;; ").map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::RegEx(f))
                            .collect::<Vec<FilterKind>>()
                    })
                {
                    container.filters.append(&mut raw_filters);
                }
            }
            Self::REGEX => {
                if let Some(mut regex_filters) =
                    parse_with_delim(new_val.as_str(), " ;;; ").map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::RegEx(f))
                            .collect::<Vec<FilterKind>>()
                    })
                {
                    container.filters.append(&mut regex_filters);
                }
            }
            _ => return Err("not a valid key".into()),
        }
        Ok(())
    }
}

pub fn filter_seq_items() -> [SeqFrame; FILTER_SEQ_NUM_FRAMES] {
    [
        SeqFrame::new(FilterSeq::EXTS, FilterSeq::EXTS_ERR, |_| true),
        SeqFrame::new(FilterSeq::FILE_TYPE, FilterSeq::FILE_TYPE_ERR, |input| {
            input.trim().len() == 0
                || ["file", "files", "dir", "dirs", "directory", "directories"]
                    .contains(&input.trim())
        }),
        SeqFrame::new(FilterSeq::RAW, FilterSeq::RAW_ERR, |_| true),
        SeqFrame::new(FilterSeq::REGEX, FilterSeq::REGEX_ERR, |input| {
            input.trim().len() == 0 || regex::Regex::new(input).is_ok()
        }),
    ]
}
