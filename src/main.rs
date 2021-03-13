pub(crate) mod apps;
pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod prelude;
pub(crate) mod tui;
pub(crate) mod utils;

use config::types::{GeneratedCommands, GlobalConfig};
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
    let cmds_path = var_os("CMDS_PATH").seppuku("CMDS PATH MISSING");
    let cmds_file = read_to_string(&cmds_path)?;
    let cmds = static_cmds(&cmds_file)?;

    let flurry_app = &mut cli::app::cli_root();

    match var_os("CFG_PATH") {
        Some(cfg_path) => {
            let cfg_file = read_to_string(cfg_path)?;
            let cfg = global_config(&cfg_file).ok();
            cli::init::exec_cli(flurry_app, cmds, cfg)
        }
        None => cli::init::exec_cli(flurry_app, cmds, None),
    }
}
