use crate::{
    config::types::{GeneratedCommands, GlobalConfig},
    prelude::*,
};

pub fn exec_cli(
    app: &mut clap::App,
    gen_cmds: GeneratedCommands,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    let matches = app.get_matches_from_safe_borrow(std::env::args_os())?;

    match matches.subcommand() {
        ("add", Some(add_args)) => crate::config::write::insert_new_cmd(add_args, gen_cmds)?,
        ("import", Some(import_args)) => crate::apps::import::import_cmds_from_file(import_args)?,
        ("export", Some(export_args)) => crate::apps::export::export_gen_cmds(export_args)?,
        ("goto", Some(goto_args)) => {
            use crate::apps::cmd::{dispatch_from_matches, interactive};
            if goto_args.is_present("interactive-mode") {
                interactive(goto_args, gen_cmds, cfg)
            } else {
                dispatch_from_matches(goto_args, gen_cmds, cfg)
            }?;
        }
        ("play", Some(play_args)) => {
            use crate::apps::play::*;

            if play_args.is_present("interactive-mode") {
                interactive(play_args, cfg)
            } else {
                exec_media_from_matches(play_args, cfg)
            }?;
        }
        ("rm", Some(rm_args)) => {
            use crate::apps::rm::{interactive, try_rm_cmd};
            if rm_args.is_present("interactive-mode") {
                interactive(gen_cmds)?;
            } else {
                try_rm_cmd(rm_args, gen_cmds)?;
                println!("Command removed");
            }
        }
        _ => app.print_long_help()?,
    }
    Ok(())
}
