use crate::{
    cli::types::{GoCmd, ProgKind},
    config::types::*,
    prelude::*,
    utils::programs::browser::{encode_url, run::web_query, WebBrowser},
};

fn unwrap_browser_infallible(
    browser_query: Option<ProgKind>,
    cfg: &Option<GlobalConfig>,
) -> WebBrowser {
    if let Some(ProgKind::Web(Some(program))) = browser_query {
        program
    } else {
        if let Some(config) = cfg {
            WebBrowser::default_from_config(config)
        } else {
            None
        }
        .unwrap_or_default()
    }
}

pub fn dispatch_from_args(
    args: GoCmd,
    cmds: GeneratedCommands,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    match &args.program {
        Some(ProgKind::Generic(Some(util))) => {
            if let Some(utils) = cmds.utils {
                for u in &utils {
                    if u == util {
                        todo!("finish util impl");
                        return Ok(());
                    }
                }
            }
        }
        Some(_) => {
            if let Some(cmds) = cmds.commands {
                for cmd in &cmds {
                    if cmd.key.eq_ignore_ascii_case(&args.command) {
                        return dispatch_command(args, cmd, &cfg);
                    }
                }
            }
        }
        None => {}
    }
    bail!("No command with that trigger")
}

fn dispatch_command(
    GoCmd { program, args, .. }: GoCmd,
    cmd: &GeneratedCommand,
    cfg: &Option<GlobalConfig>,
) -> Result<()> {
    match cmd.command_type {
        CommandType::Url => {
            let program = unwrap_browser_infallible(program, cfg);
            open_target_url(&program, cmd.target, cfg)
        }
        CommandType::WebQuery => {
            let program = unwrap_browser_infallible(program, cfg);
            open_target_url_with_queries(&program, cmd.target, &args, cfg)
        }
        CommandType::Util { .. } => unreachable!(),
    }
}

fn open_target_url<S: AsRef<str>>(
    program: &WebBrowser,
    url: S,
    cfg: &Option<GlobalConfig>,
) -> Result<()> {
    let url = encode_url(url);
    if let Some(config) = cfg {
        program.try_exec_override(url, config)
    } else {
        web_query(&program, url)
    }
}

fn open_target_url_with_queries<S: AsRef<str>>(
    program: &WebBrowser,
    url: S,
    args: &Vec<String>,
    cfg: &Option<GlobalConfig>,
) -> Result<()> {
    let url = encode_url(format!("{}{}", url.as_ref(), args.join(" ")));
    if let Some(config) = cfg {
        program.try_exec_override(url, config)
    } else {
        run_cmd!(@ program.get_bin(); url)
    }?;
    Ok(())
}

pub fn interactive_go(
    GoCmd { program, args, .. }: GoCmd,
    mut cmds: GeneratedCommands,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    use crate::tui::prelude::*;

    if let Some(ref mut cmds_list) = cmds.commands {
        let cmds_list = RefCell::from(cmds_list);

        let input_handler = TuiInputHandler::default();
        let program = unwrap_browser_infallible(program, &cfg);

        let opener = TuiCallback::Halting(|index| {
            let cmd = &cmds_list.borrow()[index];
            match cmd.command_type {
                CommandType::Url => open_target_url(&program, cmd.target, &cfg),
                CommandType::WebQuery => {
                    open_target_url_with_queries(&program, cmd.target, &args, &cfg)
                }
                CommandType::Util { .. } => unreachable!(),
            }
            .seppuku(None)
        });

        let term_opts = TuiOpts::new(input_handler, opener);
        render(term_opts, &cmds_list)?;
        return Ok(());
    }
    bail!("No commands yet!")
}
