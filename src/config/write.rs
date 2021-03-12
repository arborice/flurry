use crate::{config::types::*, prelude::*};
use std::fs::create_dir_all;

pub fn assert_config_exists() -> Result<()> {
	if !ConfigPath::Base.abs().exists() {
		setup_config_path()
	} else {
		Ok(())
	}
}

fn comment_all(content: String) -> String {
	content
		.lines()
		.fold(String::new(), |c, ln| c + "#" + ln + "\n")
}

fn setup_config_path() -> Result<()> {
	let init_conf = GlobalConfig::default();
	let ser_conf = toml::to_string(&init_conf)?;
	let commented_conf = comment_all(ser_conf);

	create_dir_all(ConfigPath::Base.abs())?;
	write(ConfigPath::Config.abs(), commented_conf)?;

	let init_cmds = GeneratedCommands::default();
	let ser_cmds = toml::to_string(&init_cmds)?;
	let commented_cmds = comment_all(ser_cmds);
	write(ConfigPath::Commands.abs(), commented_cmds)?;

	Ok(())
}

pub fn over_write_cmds(new_cmds: GeneratedCommands) -> Result<()> {
	let serialized = toml::to_string(&new_cmds)?;
	write(ConfigPath::Commands.abs(), serialized)?;
	Ok(())
}

pub fn insert_new_cmd(matches: &clap::ArgMatches) -> Result<()> {
	let key = matches
		.value_of("name")
		.ok_or(anyhow!("No key value provided"))?;
	let target = matches
		.value_of("target")
		.ok_or(anyhow!("No target value provided"))?;
	let cmd_type = match matches.value_of("type") {
		Some("util") => CommandType::Util,
		Some("web-query") => CommandType::WebQuery,
		_ => CommandType::Url,
	};

	let cmds_file = ConfigPath::Commands.try_fetch()?;
	let mut gen_cmds: GeneratedCommands = toml::from_str(&cmds_file)?;

	gen_cmds.commands = if let Some(mut cmds) = gen_cmds.commands {
		if cmds.iter().any(|cmd| key.eq_ignore_ascii_case(cmd.key)) {
			seppuku!("A command by that key is in the database!");
		}
		cmds.push(GeneratedCommand {
			key: &key,
			target,
			cmd_type,
		});
		Some(cmds)
	} else {
		Some(vec![GeneratedCommand {
			key: &key,
			target,
			cmd_type,
		}])
	};
	over_write_cmds(gen_cmds)?;
	info!("green"; f"Added {}", key);
	Ok(())
}
