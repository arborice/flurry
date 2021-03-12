use crate::prelude::*;

pub fn exec_cli(app: &mut clap::App) -> Result<()> {
    let matches = app.get_matches_from_safe_borrow(std::env::args_os())?;

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            use crate::config::write::{assert_config_exists, insert_new_cmd};
            assert_config_exists()?;
            insert_new_cmd(add_matches)?;
        }
        ("export", Some(export_matches)) => {
            use crate::apps::export::export_gen_cmds;
            let output_path = export_matches.value_of("output-file");
            export_gen_cmds(output_path)?;
            info!("Export success");
        }
        ("goto", Some(goto_matches)) => {
            use crate::{
                apps::cmd::{dispatch_from_matches, interactive},
                config::write::assert_config_exists,
            };
            assert_config_exists()?;
            if goto_matches.is_present("interactive-mode") {
                interactive()
            } else {
                dispatch_from_matches(goto_matches)
            }?;
        }
        ("import", Some(import_matches)) => {
            use crate::apps::import::import_cmds_from_file;
            let import_path = import_matches.value_of("file").seppuku("File is required!");
            import_cmds_from_file(import_path)?;
            info!("Commands imported");
        }
        ("play", Some(play_matches)) => {
            use crate::{apps::play::*, config::write::assert_config_exists};
            assert_config_exists()?;

            if play_matches.is_present("interactive-mode") {
                interactive(play_matches)
            } else {
                exec_media_from_matches(play_matches)
            }?;
        }
        ("rm", Some(rm_matches)) => {
            use crate::{
                apps::rm::{interactive, try_rm_cmd},
                config::write::assert_config_exists,
            };
            assert_config_exists()?;
            if rm_matches.is_present("interactive-mode") {
                interactive()?;
            } else {
                try_rm_cmd(rm_matches)?;
                info!("Command removed");
            }
        }
        _ => app.print_long_help()?,
    }
    Ok(())
}
