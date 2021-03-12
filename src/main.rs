pub(crate) mod apps;
pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod prelude;
pub(crate) mod tui;
pub(crate) mod utils;

fn main() {
    let flurry_app = &mut cli::app::cli_root();
    if let Err(e) = cli::init::exec_cli(flurry_app) {
        eprintln!("{}", e);
    }
}
