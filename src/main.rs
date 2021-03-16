pub(crate) mod apps;
pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod prelude;
pub(crate) mod tui;
pub(crate) mod utils;

use prelude::*;

fn global_config<'global>(ser_cfg: &'global str) -> Result<GlobalConfig<'global>> {
	let config: GlobalConfig = toml::from_str(ser_cfg)?;
	Ok(config)
}

fn static_cmds<'global>(ser_cmds: &'global str) -> Result<GeneratedCommands<'global>> {
	let cmds: GeneratedCommands = toml::from_str(ser_cmds)?;
	Ok(cmds)
}

fn main() -> Result<()> {
	use std::env::var_os;
	let cmds_path = var_os("FLURRY_CMDS_PATH").unwrap_or(ConfigPath::Commands.abs().into());
	let cmds_file: String = match read_to_string(cmds_path) {
		Ok(file) => file,
		Err(e) if e.kind() == FileNotFound => {
			config::write::init_cmds_if_not_exists()?;
			String::new()
		}
		Err(e) => seppuku!(e),
	};
	let cmds = static_cmds(&cmds_file)?;

	let flurry_app: cli::types::Flurry = argh::from_env();
	match var_os("FLURRY_CFG_PATH") {
		Some(cfg_path) => {
			let cfg_file = read_to_string(cfg_path)?;
			let cfg = global_config(&cfg_file).ok();
			cli::exec_cli(flurry_app, cmds, cfg)
		}
		None => {
			if let Ok(cfg_file) = read_to_string(ConfigPath::Config.abs()) {
				let cfg = global_config(&cfg_file).ok();
				cli::exec_cli(flurry_app, cmds, cfg)
			} else {
				cli::exec_cli(flurry_app, cmds, None)
			}
		}
	}
}
