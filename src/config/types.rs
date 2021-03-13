use crate::tui::prelude::ListEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct ProgramOverride<'a> {
	pub cmd: &'a str,
	pub args: Option<Vec<&'a str>>,
}

#[derive(Deserialize, Serialize)]
pub struct MediaPlayerOverride<'a> {
	pub bin: &'a str,
	pub args: Option<Vec<&'a str>>,
}

#[derive(Deserialize, Serialize)]
pub struct GenericUtil<'a> {
	pub bin: &'a str,
	pub args: Option<Vec<&'a str>>,
	pub aliases: Option<Vec<&'a str>>,
}

#[derive(Deserialize, Serialize)]
pub struct GlobalConfig<'a> {
	pub default_browser: Option<&'a str>,
	#[serde(borrow, rename = "player")]
	pub media_players: Option<HashMap<&'a str, MediaPlayerOverride<'a>>>,
	#[serde(borrow, rename = "override")]
	pub overrides: Option<Vec<ProgramOverride<'a>>>,
}

impl<'a> Default for GlobalConfig<'a> {
	fn default() -> GlobalConfig<'a> {
		let mut media_players = HashMap::new();
		media_players.insert(
			"audio",
			MediaPlayerOverride {
				bin: "audio-player-bin",
				args: None,
			},
		);
		media_players.insert(
			"image",
			MediaPlayerOverride {
				bin: "/path/to/image-viewer",
				args: None,
			},
		);
		media_players.insert(
			"video",
			MediaPlayerOverride {
				bin: "video-player-bin",
				args: Some(vec!["optional", "args"]),
			},
		);
		let sample_override = ProgramOverride {
			cmd: "program-bin or /path/to/bin",
			args: Some(vec!["optional", "args"]),
		};
		GlobalConfig {
			default_browser: Some("browser-bin or /path/to/browser"),
			media_players: Some(media_players),
			overrides: Some(vec![sample_override]),
		}
	}
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CommandType {
	Url,
	Util,
	WebQuery,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct GeneratedCommand<'a> {
	pub key: &'a str,
	pub target: &'a str,
	#[serde(rename = "type")]
	pub cmd_type: CommandType,
}

impl<'a> ListEntry for GeneratedCommand<'a> {
	fn as_context(&self) -> &str {
		self.key
	}

	fn as_entry(&self) -> String {
		format!("{} >> {}", self.key, self.target)
	}
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GeneratedCommands<'a> {
	#[serde(borrow, rename = "command")]
	pub commands: Option<Vec<GeneratedCommand<'a>>>,
}

impl<'a> Default for GeneratedCommands<'a> {
	fn default() -> GeneratedCommands<'a> {
		let sample_cmd = GeneratedCommand {
			key: "duck",
			target: "https://www.duckduckgo.com",
			cmd_type: CommandType::Url,
		};
		GeneratedCommands {
			commands: Some(vec![sample_cmd]),
		}
	}
}
