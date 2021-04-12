use crate::{prelude::*, tui::widgets::*, utils::parse::*};

pub const EDIT_SEQ_NUM_FRAMES: usize = 7;

pub struct EditSeq;
impl EditSeq {
    const BIN: &'static str = UiStack::BIN;
    const ARGS: &'static str = UiStack::ARGS;
    const ALIASES: &'static str = UiStack::ALIASES;
    const ENCODER: &'static str = UiStack::ENCODER;
    const PERMISSIONS: &'static str = UiStack::PERMISSIONS;
    const SCAN_DIR: &'static str = UiStack::SCAN_DIR;
    const WHICH: &'static str = UiStack::WHICH;

    const BIN_ERR: &'static str = UiStack::BIN_ERR;
    const ARGS_ERR: &'static str = UiStack::ARGS_ERR;
    const ALIASES_ERR: &'static str = UiStack::ALIASES_ERR;
    const ENCODER_ERR: &'static str = UiStack::ENCODER_ERR;
    const PERMISSIONS_ERR: &'static str = UiStack::PERMISSIONS_ERR;
    const WHICH_ERR: &'static str = UiStack::WHICH_ERR;

    pub fn set_new_val(
        key: &str,
        new_val: &String,
        GeneratedCommand {
            bin,
            dfl_args,
            encoder,
            aliases,
            permissions,
            query_which,
            scan_dir,
            ..
        }: &mut GeneratedCommand,
    ) -> Result<(), String> {
        match key {
            Self::BIN => {
                if new_val.chars().any(|c| c.is_ascii_whitespace()) {
                    return Err("".into());
                }
                *bin = new_val.to_owned();
            }
            Self::ARGS => {
                dfl_args.replace(args_from_arg(new_val.as_str())?);
            }
            Self::ALIASES => {
                aliases.replace(aliases_from_arg(new_val.as_str())?);
            }
            Self::ENCODER => {
                encoder.replace(encoder_from_arg(new_val.as_str())?);
            }
            Self::PERMISSIONS => {
                *permissions = permissions_from_arg(&new_val.as_str())?;
            }
            Self::SCAN_DIR => {
                *scan_dir = recursion_limit_from_arg(new_val.as_str())?;
            }
            Self::WHICH => {
                *query_which = match new_val.to_lowercase().trim() {
                    "y" | "yes" | "true" => true,
                    _ => false,
                }
            }
            _ => return Err("not a valid key".into()),
        }
        Ok(())
    }
}

pub fn edit_seq_items() -> [SeqFrame; EDIT_SEQ_NUM_FRAMES] {
    [
        SeqFrame::new(EditSeq::BIN, EditSeq::BIN_ERR, |val| !val.trim().is_empty()),
        SeqFrame::new(EditSeq::ARGS, EditSeq::ARGS_ERR, |_| true),
        SeqFrame::new(EditSeq::ALIASES, EditSeq::ALIASES_ERR, |_| true),
        SeqFrame::new(
            EditSeq::ENCODER,
            EditSeq::ENCODER_ERR,
            EncoderKind::is_valid,
        ),
        SeqFrame::new(
            EditSeq::PERMISSIONS,
            EditSeq::PERMISSIONS_ERR,
            PermissionsKind::is_valid,
        ),
        SeqFrame::new(EditSeq::PERMISSIONS, EditSeq::PERMISSIONS_ERR, |val| {
            u8::from_str_radix(val, 10).is_ok() || ["max", "none", "recursive"].contains(&val)
        }),
        SeqFrame::new(
            EditSeq::WHICH,
            EditSeq::WHICH_ERR,
            GeneratedCommand::is_valid,
        ),
    ]
}
