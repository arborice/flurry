use crate::prelude::*;
use std::path::PathBuf;

pub enum ConfigPath {
	Base,
	Config,
	Commands,
}

impl ConfigPath {
	pub fn abs(&self) -> PathBuf {
		let mut path = home();
		path.push(match self {
			ConfigPath::Base => ".config/flurry",
			ConfigPath::Config => ".config/flurry/config.toml",
			ConfigPath::Commands => ".config/flurry/commands.toml",
		});
		path
	}

	pub fn try_fetch(&self) -> Result<String> {
		match self {
			ConfigPath::Base => seppuku!("Base is the config dir, not a file"),
			_ => read_to_string(self.abs()).map_err(|e| anyhow!(e)),
		}
	}
}
