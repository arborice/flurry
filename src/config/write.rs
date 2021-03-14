use crate::prelude::*;
use std::fs::create_dir_all;

fn comment_all(content: &String) -> String {
	content
		.lines()
		.fold(String::new(), |c, ln| c + "#" + ln + "\n")
}

fn setup_config_path() -> Result<String> {
	let init_conf = GlobalConfig::default();
	let ser_conf = toml::to_string(&init_conf)?;
	let commented_conf = comment_all(&ser_conf);

	create_dir_all(ConfigPath::Base.abs())?;
	write(ConfigPath::Config.abs(), commented_conf)?;

	let init_cmds = GeneratedCommands::default();
	let ser_cmds = toml::to_string(&init_cmds)?;
	let commented_cmds = comment_all(&ser_cmds);
	write(ConfigPath::Commands.abs(), commented_cmds)?;

	Ok(ser_cmds)
}

pub fn init_cfg_if_not_exists() -> Result<String> {
	if !ConfigPath::Base.abs().exists() {
		setup_config_path()
	} else {
		bail!("Could not read or write config dir. Insufficient permissions?")
	}
}

pub fn over_write_cmds(new_cmds: GeneratedCommands) -> Result<()> {
	let serialized = toml::to_string(&new_cmds)?;
	write(ConfigPath::Commands.abs(), serialized)?;
	Ok(())
}

pub fn append_cmd(new_cmd: GeneratedCommand) -> Result<()> {
	use std::{fs::OpenOptions, io::Write};

	let serialized_cmd = toml::to_vec(&new_cmd)?;
	let mut cmds_file = OpenOptions::new()
		.append(true)
		.open(ConfigPath::Commands.abs())?;

	cmds_file.write(&*serialized_cmd)?;
	cmds_file.flush()?;
	Ok(())
}
