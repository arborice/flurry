use crate::{
    config::{types::*, write::overwrite_cmds},
    prelude::*,
    tui::{prelude::*, runtime::*},
};
use rkyv::{
    core_impl::ArchivedOption, de::deserializers::AllocDeserializer, std_impl::ArchivedString,
    Deserialize,
};

pub fn dispatch_interactive(gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if gen_cmds.commands.is_none() {
        return Err(anyhow!("No commands yet!"));
    }

    if let ArchivedOption::Some(ref commands) = gen_cmds.commands {
        let cmds_list: Vec<(&ArchivedString, &ArchivedGeneratedCommand)> =
            commands.iter().collect();
        let cmds_ref = RefCell::from(&cmds_list);

        let tui_opts =
            TuiOpts::default().with_selection_highlighter(Style::default().fg(Color::Blue));

        let mut app = StatefulCmdsTable::with_items(&cmds_ref);
        let exit_status = app.render(tui_opts)?;

        if exit_status.success {
            match exit_status.last_requested_action {
                Some(TuiInputHandler::GO) => {
                    if let Some(key) = exit_status.go_request {
                        let key: &str = key.as_ref();
                        if let Some(cmd) = commands.get(key) {
                            match &cmd.dfl_args {
                                ArchivedOption::Some(args) => {
                                    run_cmd!(@ cmd.bin.as_ref() => args.iter().map(|a| a.as_ref()))
                                }
                                ArchivedOption::None => run_cmd!(@ cmd.bin.as_ref() =>),
                            }?;
                            return Ok(());
                        }
                    }
                }
                Some(TuiInputHandler::RM) => {
                    let rm_selection = exit_status.rm_selection;
                    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
                    if let Some(ref mut cmds) = gen_cmds.commands {
                        cmds.retain(|key, _| !rm_selection.iter().any(|k| k == key));
                    }
                    return overwrite_cmds(gen_cmds);
                }
                _ => {}
            }
            println!("Goodbye!");
        }
    }
    Ok(())
}
