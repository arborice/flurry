use crate::prelude::*;
use std::fs::create_dir_all;

pub fn init_cmds_if_not_exists() -> Result<()> {
	if !ConfigPath::Base.abs().exists() {
		create_dir_all(ConfigPath::Base.abs())?;
		write(
			ConfigPath::Commands.abs(),
			"# see <github link> for sample commands or create some with the cli",
		)?;
		Ok(())
	} else if !ConfigPath::Commands.abs().exists() {
		write(
			ConfigPath::Commands.abs(),
			"# see <github link> for sample commands or create some with the cli",
		)?;
		Ok(())
	} else {
		bail!("Could not read from config dir. Insufficient permissions?")
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

use crate::utils::programs::generic::UtilFromArgs;
pub fn append_util(new_util: UtilFromArgs) -> Result<()> {
	use std::{fs::OpenOptions, io::Write};

	let serialized_cmd = toml::to_vec(&new_util)?;
	let mut cmds_file = OpenOptions::new()
		.append(true)
		.open(ConfigPath::Commands.abs())?;

	cmds_file.write(&*serialized_cmd)?;
	cmds_file.flush()?;
	Ok(())
}
