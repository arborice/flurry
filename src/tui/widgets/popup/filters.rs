#[derive(Debug, Default)]
pub struct FilterOpts {
    pub depth: String,
    pub ext_filters: String,
    pub file_type_filter: String,
    pub raw_filters: String,
    pub regex_filters: String,
}

use crate::{prelude::*, tui::widgets::*};

pub const FILTER_SEQ_NUM_FRAMES: usize = 5;

pub struct FilterSeq;
impl FilterSeq {
    const DEPTH: &'static str = "";
    const EXTS: &'static str = "";
    const FILE_TYPE: &'static str = "";
    const RAW: &'static str = "";
    const REGEX: &'static str = "";

    const DEPTH_ERR: &'static str = "";
    const EXTS_ERR: &'static str = "";
    const FILE_TYPE_ERR: &'static str = "";
    const RAW_ERR: &'static str = "";
    const REGEX_ERR: &'static str = "";

    pub fn set_new_val(
        key: &str,
        new_val: &String,
        GeneratedCommand { filter, .. }: &mut GeneratedCommand,
    ) -> Result<(), String> {
        // FilterOpts {
        // depth,
        // ext_filters,
        // file_type_filter,
        // raw_filters,
        // regex_filters,
        // }) => {
        // let scan_dir = recursion_limit_from_arg(&depth).or_else(|e| bail!(e))?;
        // let mut filters = vec![];
        //
        // if let Ok(ext_filters) = exts_filter_from_arg(&ext_filters) {
        // filters.push(ext_filters);
        // }
        // if let Ok(file_type_filter) = file_type_filter_from_arg(&file_type_filter) {
        // filters.push(file_type_filter);
        // }
        // if let Some(ref mut regex_filters) =
        // parse_with_delim(regex_filters, " ;;; ").map(|mut f| {
        // f.drain(..)
        // .map(|f| FilterKind::RegEx(f))
        // .collect::<Vec<FilterKind>>()
        // })
        // {
        // filters.append(regex_filters);
        // }
        // if let Some(ref mut raw_filters) =
        // parse_with_delim(raw_filters, " ;;; ").map(|mut f| {
        // f.drain(..)
        // .map(|f| FilterKind::Raw(f))
        // .collect::<Vec<FilterKind>>()
        // })
        // {
        // filters.append(raw_filters);
        // }
        //
        // match filters.len() {
        // 0 => (scan_dir, FiltersKind::None),
        // 1 => (
        // scan_dir,
        // FiltersKind::One(filters.drain(..).nth(0).unwrap()),
        // ),
        // _ => (scan_dir, FiltersKind::Many(filters)),
        // }
        // }
        match key {
            Self::DEPTH => {}
            Self::EXTS => {}
            Self::FILE_TYPE => {}
            Self::RAW => {}
            Self::REGEX => {}
            _ => return Err("not a valid key".into()),
        }
        Ok(())
    }
}

pub fn filter_seq_items() -> [SeqFrame; FILTER_SEQ_NUM_FRAMES] {
    [
        SeqFrame::new(FilterSeq::DEPTH, FilterSeq::DEPTH_ERR, |_| true),
        SeqFrame::new(FilterSeq::EXTS, FilterSeq::EXTS_ERR, |_| true),
        SeqFrame::new(FilterSeq::FILE_TYPE, FilterSeq::FILE_TYPE_ERR, |_| true),
        SeqFrame::new(FilterSeq::RAW, FilterSeq::RAW_ERR, |_| true),
        SeqFrame::new(FilterSeq::REGEX, FilterSeq::REGEX_ERR, |_| true),
    ]
}
