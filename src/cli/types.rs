use crate::prelude::*;
use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "A tiny cli utility")]
pub struct Flurry {
    #[argh(switch, short = 'i', description = "enter tui mode")]
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
    Rm(RmCmd),
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

fn aliases_from_arg(arg: &str) -> Result<Vec<String>, String> {
    let aliases: Vec<String> = arg
        .splitn(4, ",")
        .map(|alias| alias.trim().to_lowercase())
        .collect();

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

type MaybeBin = String;

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
        option,
        short = 'p',
        description = "browser used to open target (if url or web-query)"
    )]
    pub program: Option<MaybeBin>,
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
    #[argh(
        option,
        long = "file-path",
        short = 'f',
        description = "import file path"
    )]
    pub file_path: PathBuf,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "tui", description = "Enter interactive mode")]
pub struct InteractiveMode {}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "rm", description = "Remove a generated command")]
pub struct RmCmd {
    #[argh(positional, short = 'k', description = "command name to remove")]
    pub key: String,
}
