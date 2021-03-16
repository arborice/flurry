use crate::{
    config::types::CommandType,
    prelude::*,
    utils::programs::{browser::WebBrowser, media::Player},
};
use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "A tiny cli utility")]
pub struct Flurry {
    #[argh(subcommand)]
    pub subcmd: SubCmds,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
pub enum SubCmds {
    Add(AddCmd),
    AddUtil(AddUtil),
    Export(ExportCmd),
    Go(GoCmd),
    Import(ImportCmd),
    Play(PlayCmd),
    Rm(RmCmd),
}

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "add",
    description = "Create a new generated command"
)]
pub struct AddCmd {
    #[argh(
        option,
        long = "command-type",
        short = 'c',
        from_str_fn(command_type_from_arg),
        description = "url, util, or web-query"
    )]
    pub command_type: Option<CommandType>,
    #[argh(positional, short = 'n', description = "key used to trigger command")]
    pub key: String,
    #[argh(positional, short = 't', description = "command's target value")]
    pub target: String,
}

fn command_type_from_arg(given_type: &str) -> Result<CommandType, String> {
    match given_type {
        "url" => Ok(CommandType::Url),
        "util" => Ok(CommandType::Util),
        "web-query" | "query" => Ok(CommandType::WebQuery),
        _ => Err(format!("{} is not a valid command type", given_type)),
    }
}

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "add-util",
    description = "Create a new generated command"
)]
pub struct AddUtil {
    #[argh(positional, description = "key used to trigger command")]
    pub key: String,
    #[argh(option, short = 'b', description = "key used to trigger command")]
    pub bin: String,
    #[argh(
        option,
        short = 'a',
        description = "command's target value",
        from_str_fn(aliases_from_arg)
    )]
    pub aliases: Option<Vec<String>>,
    #[argh(switch, short = 'p', description = "command's target value")]
    pub permissions: bool,
    #[argh(switch, short = 's', description = "command's target value")]
    pub scan_dir: bool,
    #[argh(switch, short = 'w', description = "command's target value")]
    pub query_which: bool,
    #[argh(option, short = 'p', description = "command's target value")]
    pub sanitizer: Option<String>,
    #[argh(positional, description = "command's target value")]
    pub args: Vec<String>,
}

fn aliases_from_arg(arg: &str) -> Result<Vec<String>, String> {
    let aliases = arg
        .split(",")
        .map(|alias| alias.trim().to_lowercase())
        .collect::<Vec<String>>();

    if aliases.is_empty() {
        Err(format!("No aliases provided!"))
    } else {
        Ok(aliases)
    }
}

use std::path::PathBuf;

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "export",
    description = "Export generated commands to a file"
)]
pub struct ExportCmd {
    #[argh(
        option,
        long = "output-file",
        short = 'o',
        default = "PathBuf::from(\"flurry_exports.toml\")",
        description = "output path"
    )]
    pub output_file: PathBuf,
}

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "go",
    description = "Exec an existing generated command"
)]
pub struct GoCmd {
    #[argh(positional, description = "command key")]
    pub command: String,
    #[argh(
        switch,
        long = "interactive-mode",
        short = 'i',
        description = "open in tui"
    )]
    pub interactive_mode: bool,
    #[argh(
        option,
        short = 'p',
        description = "browser used to open target (if url or web-query)",
        from_str_fn(program_from_arg)
    )]
    pub program: Option<ProgKind>,
    #[argh(
        switch,
        description = "randomize file order for utils with dir_scan enabled"
    )]
    pub random: bool,
    #[argh(positional, description = "additional args for command")]
    pub args: Vec<String>,
}

type MaybeBin = String;
#[derive(PartialEq)]
pub enum ProgKind {
    Generic(MaybeBin),
    Media(Player),
    Web(WebBrowser),
}

fn program_from_arg(given_prog: &str) -> Result<ProgKind, String> {
    let err_msg = format!("{} is not a valid program marker", given_prog);
    if !given_prog.contains("-") {
        return Ok(ProgKind::Generic(given_prog.to_owned()));
    }

    let mut split_arg = given_prog.splitn(2, "-");
    let prefix = split_arg.next().unwrap();
    match prefix {
        "web" | "w" => {
            let prog_query = split_arg.next().seppuku(&err_msg);
            return Ok(ProgKind::Web(
                WebBrowser::try_from_str(prog_query).ok_or(err_msg)?,
            ));
        }
        "media" | "m" => {
            let prog_query = split_arg.next().seppuku(&err_msg);
            return Ok(ProgKind::Media(Player::from_str(prog_query)));
        }
        _ => {}
    }

    Ok(ProgKind::Generic(given_prog.to_owned()))
}

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "import",
    description = "Import and append commands from a file"
)]
pub struct ImportCmd {
    #[argh(
        option,
        long = "file-path",
        short = 'f',
        description = "import file path"
    )]
    pub file_path: PathBuf,
}

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "play",
    description = "Media playlist with a better shuffling algorithm"
)]
pub struct PlayCmd {
    #[argh(
        positional,
        short = 'd',
        description = "directory path",
        default = "PathBuf::from(\".\")"
    )]
    pub directory: PathBuf,
    #[argh(
        switch,
        long = "interactive-mode",
        short = 'i',
        description = "open in tui"
    )]
    pub interactive_mode: bool,
    #[argh(
        option,
        short = 'p',
        description = "media player to open files with",
        default = "Player::Video",
        from_str_fn(player_from_arg)
    )]
    pub player: Player,
    #[argh(switch, short = 'r', description = "randomize playlist")]
    pub random: bool,
    #[argh(
        switch,
        short = 'c',
        description = "case insensitive search for filter matches"
    )]
    pub case_insensitive_filter: bool,
    #[argh(option, short = 'f', description = "filter to apply to file search")]
    pub filter: Option<String>,
}

fn player_from_arg(given_player: &str) -> Result<Player, String> {
    Ok(Player::from_str(given_player))
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "rm", description = "Remove a generated command")]
pub struct RmCmd {
    #[argh(
        switch,
        long = "interactive-mode",
        short = 'i',
        description = "open in tui"
    )]
    pub interactive_mode: bool,
    #[argh(positional, short = 'k', description = "command name to remove")]
    pub key: Option<String>,
}
