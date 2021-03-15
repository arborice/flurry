use crate::{tui::prelude::ListEntry, utils::programs::generic::GenericUtil};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct ProgramOverride<'c> {
	pub cmd: &'c str,
	pub args: Option<Vec<&'c str>>,
}

#[derive(Deserialize, Serialize)]
pub struct MediaPlayerOverride<'c> {
	pub bin: &'c str,
	pub args: Option<Vec<&'c str>>,
}

#[derive(Deserialize, Serialize)]
pub struct GlobalConfig<'c> {
	pub default_browser: Option<&'c str>,
	#[serde(borrow, rename = "player")]
	pub media_players: Option<HashMap<&'c str, MediaPlayerOverride<'c>>>,
	#[serde(borrow, rename = "override")]
	pub overrides: Option<Vec<ProgramOverride<'c>>>,
}

impl<'c> Default for GlobalConfig<'c> {
	fn default() -> GlobalConfig<'c> {
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
	/// marker only
	Util,
	WebQuery,
}

impl<'c> Default for CommandType {
	fn default() -> CommandType {
		CommandType::Url
	}
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct GeneratedCommand<'c> {
	pub key: &'c str,
	pub target: &'c str,
	#[serde(rename = "type")]
	pub command_type: CommandType,
}

impl<'c> ListEntry for GeneratedCommand<'c> {
	fn as_context(&self) -> &str {
		self.key
	}

	fn as_entry(&self) -> String {
		format!("{} >> {}", self.key, self.target)
	}
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GeneratedCommands<'c> {
	#[serde(borrow, rename = "command")]
	pub commands: Option<Vec<GeneratedCommand<'c>>>,
	#[serde(borrow, rename = "util")]
	pub utils: Option<Vec<GenericUtil<'c>>>,
}

impl<'c> Default for GeneratedCommands<'c> {
	fn default() -> GeneratedCommands<'c> {
		let sample_cmd = GeneratedCommand {
			key: "duck",
			target: "https://www.duckduckgo.com",
			command_type: CommandType::Url,
		};
		GeneratedCommands {
			commands: Some(vec![sample_cmd]),
			utils: None,
		}
	}
}
