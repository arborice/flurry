use crate::{config::types::*, prelude::*};

pub fn dispatch_from_matches(args: &clap::ArgMatches) -> Result<()> {
    let query = args
        .value_of("command")
        .ok_or(anyhow!("No query provided"))?;
    let cmds_file = ConfigPath::Commands.try_fetch()?;
    if cmds_file.trim().is_empty() {
        sudoku!(0 => "purple"; "No commands exist yet, create some with the add command");
    }

    let cmds: GeneratedCommands = toml::from_str(&cmds_file)?;
    if let Some(cmds) = cmds.commands {
        for cmd in &cmds {
            if cmd.key == query {
                return dispatch_command(args, cmd);
            }
        }
    }

    bail!("No command with that trigger")
}

fn dispatch_command(args: &clap::ArgMatches, cmd: &GeneratedCommand) -> Result<()> {
    match cmd.cmd_type {
        CommandType::Url => {
            let browser = WebBrowser::from_matches(args);
            open_target_url(browser, cmd.target)?;
        }
        CommandType::WebQuery => {
            let browser = WebBrowser::from_matches(args);
            let url_queries = args
                .values_of("queries")
                .sudoku("No web queries provided!")
                .collect::<Vec<&str>>();
            open_target_url_with_args(browser, cmd.target, url_queries)?;
        }
        CommandType::Util => todo!("goto <Util> type implementation"),
    }
    Ok(())
}

fn open_target_url<S: AsRef<str>>(browser: WebBrowser, url: S) -> Result<()> {
    let url = encode_url(url);
    browser.try_exec_override(url)
}

fn open_target_url_with_args<S: AsRef<str>>(
    browser: WebBrowser,
    url: S,
    args: Vec<&str>,
) -> Result<()> {
    let url = encode_url(format!("{}{}", url.as_ref(), args.join(" ")));
    browser.try_exec_override(url)
}

pub fn interactive() -> Result<()> {
    use crate::tui::prelude::*;

    let cmds_file = ConfigPath::Commands.try_fetch()?;
    let mut cmds: GeneratedCommands = toml::from_str(&cmds_file)?;
    if let Some(ref mut cmds_list) = cmds.commands {
        let cmds_list = RefCell::from(cmds_list);

        let input_handler = TuiInputHandler::default();
        let event_loop = Events::with_exit_triggers(&input_handler.exit);

        let opener = TuiCallback::Halting(|index| {
            let cmd = &cmds_list.borrow()[index];
            if let CommandType::Url = cmd.cmd_type {
                let browser = WebBrowser::default_from_config().unwrap_or_default();
                open_target_url(browser, cmd.target).sudoku(None);
            }
        });

        let term_opts = TuiOpts::new(input_handler, event_loop, opener)?;
        render(term_opts, &cmds_list)?;
        Ok(())
    } else {
        Err(anyhow!("No commands yet!"))
    }
}
