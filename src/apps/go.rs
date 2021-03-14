use crate::{cli::argh::GoCmd, config::types::*, prelude::*};

fn unwrap_browser_infallible(
    browser_query: Option<WebBrowser>,
    cfg: &Option<GlobalConfig>,
) -> WebBrowser {
    browser_query
        .or_else(|| {
            if let Some(config) = cfg {
                WebBrowser::default_from_config(config)
            } else {
                None
            }
        })
        .unwrap_or_default()
}

pub fn dispatch_from_args(
    args: GoCmd,
    cmds: GeneratedCommands,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    if let Some(cmds) = cmds.commands {
        for cmd in &cmds {
            if cmd.key.eq_ignore_ascii_case(&args.command) {
                return dispatch_command(args, cmd, &cfg);
            }
        }
    }

    bail!("No command with that trigger")
}

fn dispatch_command(
    GoCmd {
        browser, queries, ..
    }: GoCmd,
    cmd: &GeneratedCommand,
    cfg: &Option<GlobalConfig>,
) -> Result<()> {
    match cmd.command_type {
        CommandType::Url => {
            let browser = unwrap_browser_infallible(browser, cfg);
            open_target_url(&browser, cmd.target, cfg)
        }
        CommandType::WebQuery => {
            let browser = unwrap_browser_infallible(browser, cfg);
            open_target_url_with_args(&browser, cmd.target, &queries, cfg)
        }
        CommandType::Util { .. } => todo!("goto <Util> type implementation"),
    }
}

fn open_target_url<S: AsRef<str>>(
    browser: &WebBrowser,
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
    browser: &WebBrowser,
    url: S,
    args: &Vec<String>,
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

pub fn interactive_go(
    GoCmd {
        browser, queries, ..
    }: GoCmd,
    mut cmds: GeneratedCommands,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    use crate::tui::prelude::*;

    if let Some(ref mut cmds_list) = cmds.commands {
        let cmds_list = RefCell::from(cmds_list);

        let input_handler = TuiInputHandler::default();
        let browser = unwrap_browser_infallible(browser, &cfg);

        let opener = TuiCallback::Halting(|index| {
            let cmd = &cmds_list.borrow()[index];
            match cmd.command_type {
                CommandType::Url => open_target_url(&browser, cmd.target, &cfg),
                CommandType::WebQuery => {
                    open_target_url_with_args(&browser, cmd.target, &queries, &cfg)
                }
                CommandType::Util { .. } => todo!("goto <Util> type implementation"),
            }
            .seppuku(None)
        });

        let term_opts = TuiOpts::new(input_handler, opener);
        render(term_opts, &cmds_list)?;
        return Ok(());
    }
    bail!("No commands yet!")
}
