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
