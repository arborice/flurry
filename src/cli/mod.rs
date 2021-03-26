pub mod types;
use types::*;

use crate::prelude::*;

pub fn exec_cli(app: Flurry) -> Result<()> {
    if !app.interactive_mode && app.subcmd.is_none() {
        bail!("use the `help` subcommand for info")
    }

    let archived = CmdsDb::from_cfg()?;
    if app.interactive_mode {
        return crate::apps::interactive::dispatch_interactive(archived.archive());
    }

    use SubCmds::*;
    match app.subcmd {
        Some(Add(args)) => crate::apps::add::insert_new_cmd(args, archived.archive())?,
        Some(Import(args)) => crate::apps::import::import_cmds_from_file(args)?,
        Some(Export(args)) => crate::apps::export::export_gen_cmds(args)?,
        Some(Go(args)) => crate::apps::go::dispatch_from_args(args, archived.archive())?,
        Some(Rm(args)) => crate::apps::rm::try_rm_cmd(args, archived.archive())?,
        Some(Tui(_)) => crate::apps::interactive::dispatch_interactive(archived.archive())?,
        _ => {}
    }
    Ok(())
}
