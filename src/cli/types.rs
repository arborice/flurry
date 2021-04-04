use crate::prelude::*;
use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "A tiny cli utility")]
pub struct Flurry {
    #[argh(switch, short = 'i', description = "enter interactive mode")]
    pub interactive_mode: bool,
    #[argh(subcommand)]
    pub subcmd: Option<SubCmds>,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
pub enum SubCmds {
    Add(AddCmd),
    Export(ExportCmd),
    Go(GoCmd),
    Import(ImportCmd),
    List(ListCmd),
    Rm(RmCmd),
    Set(SetCmd),
    Tui(InteractiveMode),
}

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "add",
    description = "Create a new generated command"
)]
pub struct AddCmd {
    #[argh(positional, description = "key used to trigger command")]
    pub key: String,
    #[argh(option, short = 'b', description = "key used to trigger command")]
    pub bin: String,
    #[argh(
        option,
        short = 'a',
        description = "commma separated values (max of 4) which are inserted as aliases for this command. Also used to query if query-which enabled",
        from_str_fn(aliases_from_arg)
    )]
    pub aliases: Option<Vec<String>>,
    #[argh(
        option,
        short = 'e',
        description = "(OPTIONAL) encode output\n   options: url, json",
        from_str_fn(encoder_from_arg)
    )]
    pub encoder: Option<EncoderKind>,
    #[argh(
        switch,
        short = 'p',
        description = "require permissions check to run this command"
    )]
    pub permissions: bool,
    #[argh(
        switch,
        short = 's',
        description = "whether to append a ?recursive directory scan output as arguments to this command"
    )]
    pub scan_dir: bool,
    #[argh(
        switch,
        short = 'w',
        description = "query the target system for the binary location (or alias) instead of executing the raw value of bin"
    )]
    pub query_which: bool,
    #[argh(
        option,
        short = 'f',
        description = "add a simple unicode filter for output"
    )]
    pub filter: Option<String>,
    #[argh(
        switch,
        short = 'r',
        description = "[FLAG] apply a regex filter instead of unicode"
    )]
    pub regex: bool,
    #[argh(positional, description = "command's target value")]
    pub args: Vec<String>,
}

pub fn aliases_from_arg(arg: &str) -> Result<Vec<String>, String> {
    let aliases: Vec<String> = arg
        .splitn(5, ",")
        .take(4)
        .map(|alias| alias.trim().to_lowercase())
        .collect();

    if !aliases.is_empty() {
        Ok(aliases)
    } else {
        Err("No aliases provided!".into())
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
        short = 'o',
        default = "PathBuf::from(\"flurry_exports.db\")",
        description = "output path"
    )]
    pub output_file: PathBuf,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "list", description = "List all stored commands")]
pub struct ListCmd {}

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
        description = "randomize file order for utils with dir_scan enabled"
    )]
    pub random: bool,
    #[argh(positional, description = "additional args for command")]
    pub args: Vec<String>,
}

#[derive(FromArgs, PartialEq)]
#[argh(
    subcommand,
    name = "import",
    description = "Import and append commands from a file"
)]
pub struct ImportCmd {
    #[argh(option, short = 'f', description = "import file path")]
    pub file_path: PathBuf,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "tui", description = "Enter interactive mode")]
pub struct InteractiveMode {}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "rm", description = "Remove a generated command")]
pub struct RmCmd {
    #[argh(switch, short = 'k', description = "remove alias, not command")]
    pub alias: bool,
    #[argh(positional, short = 'k', description = "command name to remove")]
    pub key: String,
}

use crate::config::types::{EncoderKind, FileTypeFilter, FilterKind, PermissionsKind};

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "set", description = "Edit a command's attributes")]
pub struct SetCmd {
    #[argh(positional, description = "target command to edit")]
    pub target: String,
    #[argh(option, short = 'b', description = "key used to trigger command")]
    pub bin: Option<String>,
    #[argh(
        option,
        short = 'a',
        description = "add a new alias (fails if exceeds 4)"
    )]
    pub alias: Option<String>,
    #[argh(
        option,
        short = 'p',
        description = "require permissions check to run this command",
        from_str_fn(permissions_from_arg)
    )]
    pub permissions: Option<PermissionsKind>,
    #[argh(
        option,
        short = 's',
        description = "set a recursion limit for directory scan",
        from_str_fn(recursion_limit_from_arg)
    )]
    pub scan_dir_depth: Option<ScanDirKind>,
    #[argh(
        option,
        short = 'w',
        description = "query the target system for the binary location (or alias) instead of executing the raw value of bin"
    )]
    pub query_which: Option<bool>,
    #[argh(
        option,
        short = 'x',
        description = "add file extension filters for recursive directory scans",
        from_str_fn(exts_filter_from_arg)
    )]
    pub ext_filter: Option<FilterKind>,
    #[argh(
        option,
        short = 'f',
        description = "add file type filter (dirs only, files only) for recursive directory scans",
        from_str_fn(file_type_filter_from_arg)
    )]
    pub file_type_filter: Option<FilterKind>,
    #[argh(
        option,
        short = 'e',
        description = "add file type filter (dirs only, files only) for recursive directory scans",
        from_str_fn(encoder_from_arg)
    )]
    pub encoder: Option<EncoderKind>,
    #[argh(
        option,
        short = 'n',
        description = "command's target value",
        from_str_fn(args_from_arg)
    )]
    pub args: Option<Vec<String>>,
    #[argh(
        switch,
        short = 'p',
        description = "append arguments instead of replacing them"
    )]
    pub append_args: bool,
}

pub fn recursion_limit_from_arg(arg: &str) -> Result<ScanDirKind, String> {
    match arg {
        "max" | "recursive" => Ok(ScanDirKind::Depth(u8::MAX)),
        "none" => Ok(ScanDirKind::None),
        _ => match arg.parse::<u8>() {
            Ok(0) => Ok(ScanDirKind::None),
            Ok(any) => Ok(ScanDirKind::Depth(any)),
            _ => Err(format!("{} is not a valid depth", arg)),
        },
    }
}

pub fn encoder_from_arg(arg: &str) -> Result<EncoderKind, String> {
    match arg {
        "url" | "web" => Ok(EncoderKind::Url),
        "json" => Ok(EncoderKind::Json),
        _ => Err(String::from("valid inputs are url, json")),
    }
}

pub fn permissions_from_arg(arg: &str) -> Result<PermissionsKind, String> {
    match arg.trim() {
        "group" => Ok(PermissionsKind::Group),
        "user" => Ok(PermissionsKind::User),
        "root" => Ok(PermissionsKind::Root),
        "any" | "dfl" | "none" => Ok(PermissionsKind::Any),
        _ => Err(String::from("valid inputs are group, user, root, any")),
    }
}

pub fn exts_filter_from_arg(arg: &str) -> Result<FilterKind, String> {
    args_from_arg(arg)
        .map(|exts| FilterKind::Exts(exts))
        .map_err(|_| "no filters provided!".into())
}

pub fn file_type_filter_from_arg(arg: &str) -> Result<FilterKind, String> {
    match arg.trim() {
        "d" | "dir" | "dirs" | "directory" | "directories" => {
            Ok(FilterKind::FileType(FileTypeFilter::Dirs))
        }
        "f" | "file" | "files" => Ok(FilterKind::FileType(FileTypeFilter::Files)),
        _ => Err(String::from("valid inputs are d, dir, f , file")),
    }
}

pub fn args_from_arg(arg: &str) -> Result<Vec<String>, String> {
    let args: Vec<String> = arg
        .split_ascii_whitespace()
        .filter_map(|a| {
            if !a.is_empty() {
                Some(a.to_owned())
            } else {
                None
            }
        })
        .collect();

    if !args.is_empty() {
        Ok(args)
    } else {
        Err("no args provided!".into())
    }
}
