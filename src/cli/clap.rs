use clap::{App, AppSettings, Arg};

pub fn cli_root() -> App<'static, 'static> {
    App::new("flurry")
        .author("David Fajardo")
        .version(clap::crate_version!())
        .arg(
            Arg::with_name("command")
                .index(1)
                .case_insensitive(true)
                .hidden(true),
        )
        .subcommands(vec![
            App::new("add")
                .about("Create a new generated command")
                .args(&[
                    Arg::with_name("name")
                        .short("n")
                        .takes_value(true)
                        .case_insensitive(true)
                        .required(true),
                    Arg::with_name("target")
                        .short("t")
                        .takes_value(true)
                        .case_insensitive(true)
                        .required(true),
                    Arg::with_name("command-type")
                        .short("c")
                        .takes_value(true)
                        .required(false)
                        .default_value("url")
                        .possible_values(&["url", "util", "web-query"]),
                ]),
            App::new("export")
                .about("Export generated commands to a file")
                .arg(Arg::with_name("output-file").short("o").takes_value(true)),
            App::new("goto")
                .about("Exec an existing generated command")
                .alias("go")
                .args(&[
                    Arg::with_name("command")
                        .takes_value(true)
                        .case_insensitive(true)
                        .index(1),
                    Arg::with_name("interactive-mode")
                        .short("i")
                        .takes_value(false),
                    Arg::with_name("program")
                        .short("p")
                        .required(false)
                        .takes_value(true)
                        .case_insensitive(true),
                    Arg::with_name("queries")
                        .short("q")
                        .multiple(true)
                        .takes_value(true)
                        .case_insensitive(true),
                ]),
            App::new("import")
                .about("Import and append commands from a file")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .index(1)
                        .takes_value(true)
                        .required(true),
                ),
            App::new("play")
                .alias("media")
                .about("Media playlist with a better shuffling algorithm")
                .setting(AppSettings::TrailingVarArg)
                .args(&[
                    Arg::with_name("directory")
                        .short("d")
                        .index(1)
                        .takes_value(true)
                        .default_value("."),
                    Arg::with_name("interactive-mode")
                        .short("i")
                        .takes_value(false),
                    Arg::with_name("player")
                        .short("p")
                        .takes_value(true)
                        .possible_values(&["audio", "image", "video"]),
                    Arg::with_name("random")
                        .short("r")
                        .long("random")
                        .takes_value(false),
                    Arg::with_name("filter")
                        .takes_value(true)
                        .allow_hyphen_values(true),
                ]),
            App::new("rm")
                .about("Remove a generated command")
                .alias("remove")
                .args(&[
                    Arg::with_name("interactive-mode")
                        .short("i")
                        .takes_value(false),
                    Arg::with_name("key")
                        .short("k")
                        .index(1)
                        .takes_value(true)
                        .case_insensitive(true),
                ]),
        ])
}

use crate::{config::types::GeneratedCommands, prelude::*};

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
            use crate::apps::cmd::*;
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
            use crate::apps::rm::*;
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
