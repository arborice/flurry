use argh::FromArgs;
use crate::{
	config::types::{CommandType, GeneratedCommands},
	prelude::*,
	utils::programs::media::player::Player,
};

#[derive(FromArgs)]
#[argh(description = "A tiny cli utility")]
pub struct Flurry {
	#[argh(subcommand)]
	subcmd: SubCmds,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
enum SubCmds {
	Add(AddCmd),
	Export(ExportCmd),
	Go(GoCmd),
	Import(ImportCmd),
	Play(PlayCmd),
	Rm(RmCmd),
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "add", description = "Create a new generated command")]
struct AddCmd {
	#[argh(option, long = "command-type", short = 'c', from_str_fn(command_type_from_arg), description = "url, util, or web-query")]
	command_type: Option<CommandType>,
	#[argh(positional, short = 'n', description = "key used to trigger command")]
	name: String,
	#[argh(positional, short = 't', description = "command's target value")]
	target: String,
}

fn command_type_from_arg(given_type: &str) -> Result<CommandType, String> {
	match given_type {
		"url" => Ok(CommandType::Url),
		"util" => Ok(CommandType::Util),
		"web-query" | "query" => Ok(CommandType::WebQuery),
		_ => Err(format!("{} is not a valid command type", given_type))
	}
}

use std::path::PathBuf;
fn default_cmds_out() -> PathBuf {
	PathBuf::from("flurry_exports.toml")
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "export", description = "Export generated commands to a file")]
struct ExportCmd {
	#[argh(option, long = "output-file", short = 'o', default = "default_cmds_out()", description = "output path")]
	output_file: PathBuf,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "go", description = "Exec an existing generated command")]
struct GoCmd {
	#[argh(positional, description = "command key")]
	command: String,
	#[argh(switch, long = "interactive-mode", short = 'i', description = "open in tui")]
	interactive_mode: bool,
	#[argh(option, short = 'b', description = "browser used to open target (if url or web-query)", from_str_fn(web_browser_from_arg))]
	browser: Option<WebBrowser>,
	#[argh(positional, short = 'q', description = "web queries for web-query type command")]
	queries: Vec<String>,
}

fn web_browser_from_arg(given_browser: &str) -> Result<WebBrowser, String> {
	WebBrowser::try_from_str(given_browser)
		.ok_or(format!("{} is not a valid browser type", given_browser))
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "import", description = "Import and append commands from a file")]
struct ImportCmd {
	#[argh(option, short = 'f', description = "import file path")]
	file: PathBuf
}

fn default_play_path() -> PathBuf {
	PathBuf::from(".")
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "play", description = "Media playlist with a better shuffling algorithm")]
struct PlayCmd {
	#[argh(positional, short = 'd', description = "directory path")]
	directory: PathBuf,
	#[argh(switch, long = "interactive-mode", short = 'i', description = "open in tui")]
	interactive_mode: bool,
	#[argh(option, short = 'p', description = "media player to open files with", from_str_fn(player_from_arg))]
	player: Option<Player>,
	#[argh(switch, short = 'r', description = "randomize playlist")]
	random: bool,
	#[argh(positional, short = 'f', description = "filter to apply to file search")]
	filter: Vec<String>,
}

fn player_from_arg(given_player: &str) -> Result<Player, String> {
	Ok(Player::from_str(given_player))
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "rm", description = "Remove a generated command")]
struct RmCmd {
	#[argh(switch, long = "interactive-mode", short = 'i', description = "open in tui")]
	interactive_mode: bool,
	#[argh(positional, short = 'k', description = "command name to remove")]
	key: String,
}

pub fn exec_cli(
	app: Flurry,
	gen_cmds: GeneratedCommands,
	cfg: Option<GlobalConfig>
) -> Result<()> {
	use SubCmds::*;
	match app.subcmd {
		Add(args) => crate::config::write::insert_new_cmd(args, gen_cmds)?,
		Import(args) => crate::apps::import::import_cmds_from_file(args)?,
		Export(args) => crate::apps::export::export_gen_cmds(args)?,
		Go(args) => {
			use crate::apps::cmd::*;
			if args.interactive_mode {
				interactive(args, gen_cmds, cfg)
			} else {
				dispatch_from_matches(args, gen_cmds, cfg)
			}?;
		}
		Play(args) => {
			use crate::apps::play::*;
			if args.interactive_mode {
				interactive(args, cfg)
			} else {
				exec_media_from_matches(args, cfg)
			}?;
		}
		Rm(args) => {
			use crate::apps::rm::*;
			if args.interactive_mode {
				interactive(gen_cmds)
			} else {
				try_rm_cmd(args, gen_cmds)
			}?;
		}
	}
	Ok(())
}