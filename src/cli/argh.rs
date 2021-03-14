use crate::{
	config::types::{CommandType, GeneratedCommands},
	prelude::*,
	utils::programs::media::player::Player,
};
use argh::FromArgs;

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
		"util" => Ok(CommandType::default()),
		"web-query" | "query" => Ok(CommandType::WebQuery),
		u if u.ends_with("util") => {
			let (mut scan_dir, mut permissions, mut query_which) = (false, false, false);
			let settings = u
				.split("-")
				.nth(0)
				.seppuku(format!("{} is not a valid util type", given_type));
			for setting in settings.chars() {
				match setting {
					's' => scan_dir = true,
					'p' => permissions = true,
					'w' => query_which = true,
					_ => return Err(format!("{} is not a valid util setting", setting)),
				}
			}
			Ok(CommandType::Util {
				scan_dir,
				permissions,
				query_which,
			})
		}
		_ => Err(format!("{} is not a valid command type", given_type)),
	}
}

#[derive(FromArgs, PartialEq)]
#[argh(
	subcommand,
	name = "add",
	description = "Create a new generated command"
)]
pub struct AddUtil {
	#[argh(positional, short = 'n', description = "key used to trigger command")]
	pub key: String,
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
		.map(|alias| {
			let mut alias = String::from(alias);
			alias.make_ascii_lowercase();
			alias
		})
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
		short = 'b',
		description = "browser used to open target (if url or web-query)",
		from_str_fn(web_browser_from_arg)
	)]
	pub browser: Option<WebBrowser>,
	#[argh(
		positional,
		short = 'q',
		description = "web queries for web-query type command"
	)]
	pub queries: Vec<String>,
}

fn web_browser_from_arg(given_browser: &str) -> Result<WebBrowser, String> {
	WebBrowser::try_from_str(given_browser)
		.ok_or(format!("{} is not a valid browser type", given_browser))
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

pub fn exec_cli(app: Flurry, gen_cmds: GeneratedCommands, cfg: Option<GlobalConfig>) -> Result<()> {
	use SubCmds::*;
	match app.subcmd {
		Add(args) => crate::apps::add::insert_new_cmd(args, gen_cmds)?,
		Import(args) => crate::apps::import::import_cmds_from_file(args)?,
		Export(args) => crate::apps::export::export_gen_cmds(args)?,
		Go(args) => {
			use crate::apps::go::*;
			if args.interactive_mode {
				interactive_go(args, gen_cmds, cfg)
			} else {
				dispatch_from_args(args, gen_cmds, cfg)
			}?;
		}
		Play(args) => {
			use crate::apps::play::*;
			if args.interactive_mode {
				interactive_play(args, cfg)
			} else {
				exec_media_from_args(args, cfg)
			}?;
		}
		Rm(args) => {
			use crate::apps::rm::*;
			if args.interactive_mode {
				interactive_rm(gen_cmds)
			} else {
				try_rm_cmd(args, gen_cmds)
			}?;
		}
	}
	Ok(())
}
