use crate::{
    prelude::*,
    tui::{layout::*, widgets::*},
    utils::parse::*,
};

pub const ADD_SEQ_NUM_ITEMS: usize = 8;

pub struct AddSeq;
impl AddSeq {
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

pub fn add_seq_items() -> [SeqFrame; ADD_SEQ_NUM_ITEMS] {
    [
        SeqFrame::new(UiStack::KEY, UiStack::KEY_ERR, |val| !val.trim().is_empty()),
        SeqFrame::new(UiStack::BIN, UiStack::BIN_ERR, |val| !val.trim().is_empty()),
        SeqFrame::new(UiStack::ARGS, UiStack::ARGS_ERR, |_| true),
        SeqFrame::new(UiStack::ALIASES, UiStack::ALIASES_ERR, |_| true),
        SeqFrame::new(
            UiStack::ENCODER,
            UiStack::ENCODER_ERR,
            EncoderKind::is_valid,
        ),
        SeqFrame::new(
            UiStack::PERMISSIONS,
            UiStack::PERMISSIONS_ERR,
            PermissionsKind::is_valid,
        ),
        SeqFrame::new(
            UiStack::SCAN_DIR,
            UiStack::SCAN_DIR_ERR,
            GeneratedCommand::is_valid,
        ),
        SeqFrame::new(
            UiStack::WHICH,
            UiStack::WHICH_ERR,
            GeneratedCommand::is_valid,
        ),
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

#[derive(Debug, Default)]
struct CmdToAdd {
    key: String,
    bin: String,
    joined_args: String,
    joined_aliases: String,
    encoder: String,
    permissions: String,
    query_which: String,
    scan_dir: String,
}

impl UiStackSequence<8_usize> {
    pub fn into_cmd(&mut self) -> Result<(String, GeneratedCommand)> {
        let transposed = CmdToAdd {
            key: self.stages[0].buf.drain(..).collect(),
            bin: self.stages[1].buf.drain(..).collect(),
            joined_args: self.stages[2].buf.drain(..).collect(),
            joined_aliases: self.stages[3].buf.drain(..).collect(),
            encoder: self.stages[4].buf.drain(..).collect(),
            permissions: self.stages[5].buf.drain(..).collect(),
            query_which: self.stages[6].buf.drain(..).collect(),
            scan_dir: self.stages[7].buf.drain(..).collect(),
        };

        let aliases = aliases_from_arg(&transposed.joined_aliases).ok();
        let dfl_args = args_from_arg(&transposed.joined_args).ok();
        let encoder = encoder_from_arg(&transposed.encoder).ok();
        let permissions = permissions_from_arg(&transposed.permissions).or_else(|e| bail!(e))?;
        let scan_dir = recursion_limit_from_arg(&transposed.scan_dir).unwrap_or_default();
        let query_which = match transposed.query_which.as_str() {
            "y" | "yes" | "true" => true,
            "f" | "n" | "no" | "false" => false,
            _ => return Err(anyhow!("not a valid input!")),
        };

        Ok((
            transposed.key,
            GeneratedCommand {
                aliases,
                bin: transposed.bin,
                dfl_args,
                encoder,
                filter: FiltersKind::None,
                permissions,
                query_which,
                scan_dir,
            },
        ))
    }
}
