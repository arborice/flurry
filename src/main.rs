pub(crate) mod apps;
pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod prelude;
pub(crate) mod tui;
pub(crate) mod utils;

use prelude::*;

fn main() -> Result<()> {
    config::write::init_cmds_if_not_exists()?;

    let flurry_app: cli::types::Flurry = argh::from_env();
    cli::exec_cli(flurry_app)
}
