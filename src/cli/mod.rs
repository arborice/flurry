pub mod types;
use types::*;

use crate::{apps, prelude::*};

pub fn exec_cli(app: Flurry) -> Result<()> {
    if !app.interactive_mode && app.subcmd.is_none() {
        return Err(anyhow!("try `flurry help` for usage info"));
    }

    use SubCmds::*;
    if let Some(Export(args)) = app.subcmd {
        return apps::export::export_gen_cmds(args);
    }

    let cmds_db = CmdsDb::from_cfg()?;
    let cmds_rkyv = cmds_db.archive();
    if app.interactive_mode {
        return apps::interactive::dispatch_interactive(cmds_rkyv);
    }

    match app.subcmd {
        Some(Add(args)) => apps::add::insert_new_cmd(args, cmds_rkyv)?,
        Some(Import(args)) => apps::import::import_cmds_from_file(args, cmds_rkyv)?,
        Some(Go(args)) => apps::go::dispatch_from_args(args, cmds_rkyv)?,
        Some(List(args)) => apps::list::list_cmds(args, cmds_rkyv),
        Some(Rm(args)) => apps::rm::try_rm_cmd(args, cmds_rkyv)?,
        Some(Set(args)) => apps::set_attr::edit_cmd(args, cmds_rkyv)?,
        Some(Tui(_)) => apps::interactive::dispatch_interactive(cmds_rkyv)?,
        _ => {}
    }
    Ok(())
}
