pub mod types;
use types::*;

use crate::prelude::*;

pub fn exec_cli(app: Flurry, gen_cmds: GeneratedCommands) -> Result<()> {
    if app.interactive_mode {
        return crate::apps::interactive::dispatch_interactive(gen_cmds);
    }

    use SubCmds::*;
    match app.subcmd {
        Some(Add(args)) => crate::apps::add::insert_new_cmd(args, gen_cmds)?,
        Some(Import(args)) => crate::apps::import::import_cmds_from_file(args)?,
        Some(Export(args)) => crate::apps::export::export_gen_cmds(args)?,
        Some(Go(args)) => crate::apps::go::dispatch_from_args(args, gen_cmds)?,
        Some(Rm(args)) => crate::apps::rm::try_rm_cmd(args, gen_cmds)?,
        Some(Tui(_)) => crate::apps::interactive::dispatch_interactive(gen_cmds)?,
        _ => {}
    }
    Ok(())
}
