use crate::{prelude::*, tui::widgets::*, utils::parse::*};

pub const fn edit_seq_num_frames() -> usize {
    7
}

pub struct EditSeq;
impl EditSeq {
    pub const BIN: &'static str = "0";
    pub const ARGS: &'static str = "1";
    pub const ALIASES: &'static str = "2";
    pub const ENCODER: &'static str = "3";
    pub const PERMISSIONS: &'static str = "4";
    pub const WHICH: &'static str = "5";
    pub const FILTERS: &'static str = "6";

    pub fn set_new_val(
        key: &str,
        new_val: &mut String,
        cmd: &mut GeneratedCommand,
    ) -> Result<(), String> {
        match key {
            Self::BIN => {
                if new_val.chars().any(|c| c.is_ascii_whitespace()) {
                    return Err("".into());
                }
                cmd.bin = new_val.clone();
            }
            Self::ARGS => {
                cmd.dfl_args.replace(args_from_arg(new_val.as_str())?);
            }
            Self::ALIASES => {
                cmd.aliases.replace(aliases_from_arg(new_val.as_str())?);
            }
            Self::ENCODER => {}
            Self::PERMISSIONS => {}
            Self::WHICH => {}
            Self::FILTERS => {}
            _ => return Err("not a valid key".into()),
        }
        Ok(())
    }
}

pub fn edit_seq_items() -> [SeqFrame; edit_seq_num_frames()] {
    [
        SeqFrame::new(EditSeq::BIN, "", |_| true),
        SeqFrame::new(EditSeq::ARGS, "", |_| true),
        SeqFrame::new(EditSeq::ALIASES, "", |_| true),
        SeqFrame::new(EditSeq::ENCODER, "", |_| true),
        SeqFrame::new(EditSeq::PERMISSIONS, "", |_| true),
        SeqFrame::new(EditSeq::WHICH, "", |_| true),
        SeqFrame::new(EditSeq::FILTERS, "", |_| true),
    ]
}
