use crate::{prelude::*, tui::widgets::*, utils::parse::*};

pub const FILTER_SEQ_NUM_FRAMES: usize = 4;

pub struct FilterContainer {
    filters: Vec<FilterKind>,
    pub ready: bool,
}

impl Default for FilterContainer {
    fn default() -> Self {
        Self {
            filters: Vec::with_capacity(3),
            ready: false,
        }
    }
}

impl FilterContainer {
    fn append(&mut self, new_filters: &mut Vec<FilterKind>) {
        self.filters.append(new_filters)
    }

    fn push(&mut self, new_filter: FilterKind) {
        self.filters.push(new_filter)
    }

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
            1 => FiltersKind::One(self.filters.swap_remove(0)),
            _ => FiltersKind::Many(self.filters.drain(..).collect()),
        };
        self.ready = false;
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
                container.push(exts_filter);
            }
            Self::FILE_TYPE => {
                let file_type_filter = file_type_filter_from_arg(new_val.as_str())?;
                container.push(file_type_filter);
            }
            Self::RAW => {
                let mut raw_filters = parse_with_delim(new_val.as_str(), " ;;; ")
                    .map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::RegEx(f))
                            .collect::<Vec<FilterKind>>()
                    })
                    .ok_or("".to_string())?;
                container.append(&mut raw_filters);
            }
            Self::REGEX => {
                let mut regex_filters = parse_with_delim(new_val.as_str(), " ;;; ")
                    .map(|mut f| {
                        f.drain(..)
                            .map(|f| FilterKind::RegEx(f))
                            .collect::<Vec<FilterKind>>()
                    })
                    .ok_or("".to_string())?;
                container.append(&mut regex_filters);
            }
            _ => return Err("not a valid key".into()),
        }
        Ok(())
    }
}

pub fn filter_seq_items() -> [SeqFrame; FILTER_SEQ_NUM_FRAMES] {
    [
        SeqFrame::new(FilterSeq::EXTS, FilterSeq::EXTS_ERR, |_| true),
        SeqFrame::new(FilterSeq::FILE_TYPE, FilterSeq::FILE_TYPE_ERR, |_| true),
        SeqFrame::new(FilterSeq::RAW, FilterSeq::RAW_ERR, |_| true),
        SeqFrame::new(FilterSeq::REGEX, FilterSeq::REGEX_ERR, |_| true),
    ]
}
