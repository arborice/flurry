use crate::{config::types::*, prelude::*};

fn safe_unwrap_browser(args: &clap::ArgMatches, cfg: &Option<GlobalConfig>) -> WebBrowser {
    WebBrowser::from_matches(args)
        .or_else(|| {
            if let Some(config) = cfg {
                WebBrowser::default_from_config(config)
            } else {
                None
            }
        })
        .unwrap_or_default()
}

pub fn dispatch_from_matches(
    args: &clap::ArgMatches,
    cmds: GeneratedCommands,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    let query = args
        .value_of("command")
        .ok_or(anyhow!("No query provided"))?;

    if let Some(cmds) = cmds.commands {
        for cmd in &cmds {
            if cmd.key == query {
                return dispatch_command(args, cmd, &cfg);
            }
        }
    }

    bail!("No command with that trigger")
}

fn dispatch_command(
    args: &clap::ArgMatches,
    cmd: &GeneratedCommand,
    cfg: &Option<GlobalConfig>,
) -> Result<()> {
    match cmd.cmd_type {
        CommandType::Url => {
            let browser = safe_unwrap_browser(args, cfg);
            open_target_url(browser, cmd.target, cfg)
        }
        CommandType::WebQuery => {
            let browser = safe_unwrap_browser(args, cfg);
            let url_queries = args
                .values_of("queries")
                .seppuku("No web queries provided!")
                .collect::<Vec<&str>>();

            open_target_url_with_args(browser, cmd.target, url_queries, cfg)
        }
        CommandType::Util => todo!("goto <Util> type implementation"),
    }
}

fn open_target_url<S: AsRef<str>>(
    browser: WebBrowser,
    url: S,
    cfg: &Option<GlobalConfig>,
) -> Result<()> {
    let url = encode_url(url);
    if let Some(config) = cfg {
        browser.try_exec_override(url, config)
    } else {
        web_query(&browser, url)
    }
}

fn open_target_url_with_args<S: AsRef<str>>(
    browser: WebBrowser,
    url: S,
    args: Vec<&str>,
    cfg: &Option<GlobalConfig>,
) -> Result<()> {
    let url = encode_url(format!("{}{}", url.as_ref(), args.join(" ")));
    if let Some(config) = cfg {
        browser.try_exec_override(url, config)
    } else {
        run_cmd!(@ browser.get_bin(); url)
    }?;
    Ok(())
}

pub fn interactive(
    args: &clap::ArgMatches,
    mut cmds: GeneratedCommands,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    use crate::tui::prelude::*;

    if let Some(ref mut cmds_list) = cmds.commands {
        let cmds_list = RefCell::from(cmds_list);

        let input_handler = TuiInputHandler::default();
        let event_loop = Events::with_exit_triggers(&input_handler.exit);

        let opener = TuiCallback::Halting(|index| {
            let cmd = &cmds_list.borrow()[index];
            if let CommandType::Url = cmd.cmd_type {
                let browser = safe_unwrap_browser(args, &cfg);
                open_target_url(browser, cmd.target, &cfg).seppuku(None);
            }
        });

        let term_opts = TuiOpts::new(input_handler, event_loop, opener)?;
        render(term_opts, &cmds_list)?;
        Ok(())
    } else {
        Err(anyhow!("No commands yet!"))
    }
}
