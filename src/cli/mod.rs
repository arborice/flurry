pub mod types;
use types::*;

use crate::prelude::*;

pub fn exec_cli(app: Flurry, gen_cmds: GeneratedCommands, cfg: Option<GlobalConfig>) -> Result<()> {
    use SubCmds::*;
    match app.subcmd {
        Add(args) => crate::apps::add::insert_new_cmd(args, gen_cmds)?,
        AddUtil(args) => crate::apps::add::create_new_util(args, gen_cmds)?,
        Import(args) => crate::apps::import::import_cmds_from_file(args)?,
        Export(args) => crate::apps::export::export_gen_cmds(args)?,
        Go(args) => {
            use crate::apps::go::*;
            if args.interactive_mode {
                dispatch_interactive(args, gen_cmds, cfg)
            } else {
                dispatch_from_args(args, gen_cmds, cfg)
            }?;
        }
        Play(args) => {
            use crate::apps::play::*;
            if args.interactive_mode {
                interactive_play(args, cfg)
            } else {
                exec_media_from_args(args, cfg)
            }?;
        }
        Rm(args) => {
            use crate::apps::rm::*;
            if args.interactive_mode {
                interactive_rm(gen_cmds)
            } else {
                try_rm_cmd(args, gen_cmds)
            }?;
        }
    }
    Ok(())
}
