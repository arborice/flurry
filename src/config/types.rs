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

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CommandType {
	Url,
	Util,
	WebQuery,
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
