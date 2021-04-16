use crate::{
    prelude::*,
    tui::{layout::*, widgets::*},
    utils::parse::*,
};

pub const ADD_SEQ_NUM_FRAMES: usize = 8;

pub struct AddSeq;
impl AddSeq {
    pub const KEY: &'static str = UiStack::KEY;
    pub const BIN: &'static str = UiStack::BIN;
    pub const ARGS: &'static str = UiStack::ARGS;
    pub const ALIASES: &'static str = UiStack::ALIASES;
    pub const ENCODER: &'static str = UiStack::ENCODER;
    pub const PERMISSIONS: &'static str = UiStack::PERMISSIONS;
    pub const SCAN_DIR: &'static str = UiStack::SCAN_DIR;
    pub const WHICH: &'static str = UiStack::WHICH;

    const KEY_ERR: &'static str = UiStack::KEY_ERR;
    const BIN_ERR: &'static str = UiStack::BIN_ERR;
    const ARGS_ERR: &'static str = UiStack::ARGS_ERR;
    const ALIASES_ERR: &'static str = UiStack::ALIASES_ERR;
    const ENCODER_ERR: &'static str = UiStack::ENCODER_ERR;
    const PERMISSIONS_ERR: &'static str = UiStack::PERMISSIONS_ERR;
    const SCAN_DIR_ERR: &'static str = UiStack::SCAN_DIR_ERR;
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

pub fn add_seq_items() -> [SeqFrame; ADD_SEQ_NUM_FRAMES] {
    [
        SeqFrame::new(AddSeq::KEY, AddSeq::KEY_ERR, |val| !val.trim().is_empty()),
        SeqFrame::new(AddSeq::BIN, AddSeq::BIN_ERR, |val| !val.trim().is_empty()),
        SeqFrame::new(AddSeq::ARGS, AddSeq::ARGS_ERR, |_| true),
        SeqFrame::new(AddSeq::ALIASES, AddSeq::ALIASES_ERR, |_| true),
        SeqFrame::new(AddSeq::ENCODER, AddSeq::ENCODER_ERR, EncoderKind::is_valid),
        SeqFrame::new(
            AddSeq::PERMISSIONS,
            AddSeq::PERMISSIONS_ERR,
            PermissionsKind::is_valid,
        ),
        SeqFrame::new(
            AddSeq::SCAN_DIR,
            AddSeq::SCAN_DIR_ERR,
            GeneratedCommand::is_valid,
        ),
        SeqFrame::new(AddSeq::WHICH, AddSeq::WHICH_ERR, GeneratedCommand::is_valid),
    ]
}

pub fn centered_input_block<B: Backend>(frame: &mut Frame<B>, content: &str) {
    let input_rect = centered_rect_with_margin(65, 1, frame.size(), (Direction::Vertical, 1));
    let input_block = Block::default().borders(Borders::BOTTOM);
    frame.render_widget(input_block, input_rect);

    let disp_rect = centered_rect(65, 1, frame.size());
    let display = Paragraph::new(Spans::from(content)).wrap(Wrap { trim: false });
    frame.render_widget(display, disp_rect);
}
