use crate::{
    apps::add::commit_cmd,
    config::{types::*, write::overwrite_cmds},
    prelude::*,
    tui::{prelude::*, runtime::*, widgets::popup::add::AddCmdUi},
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

        let mut app = StatefulCmdsTable::with_items(&cmds_ref);
        let event_handler = StatefulEventHandler::new();
        let exit_status = app.render(event_handler)?;

        if exit_status.success {
            match exit_status.last_requested_action {
                Some(EventHandler::ADD) => {
                    if let Some(cmd) = exit_status.new_cmd {
                        let key = cmd.key.clone();
                        insert_new_cmd(cmd, gen_cmds)?;
                        println!("{} successfully added", key);
                    }
                }
                Some(EventHandler::GO) => {
                    if let Some(key) = exit_status.go_request {
                        if let Some(cmd) = gen_cmds.get(&key) {
                            match &cmd.dfl_args {
                                ArchivedOption::Some(args) => {
                                    run_cmd!(@ cmd.bin.as_ref() => args.iter().map(|a| a.as_ref()))
                                }
                                ArchivedOption::None => run_cmd!(@ cmd.bin.as_ref() =>),
                            }?;
                        }
                    }
                }
                _ => {}
            }

            if !exit_status.rm_selection.is_empty() {
                let selection = exit_status.rm_selection;
                let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
                if let Some(ref mut cmds) = gen_cmds.commands {
                    cmds.retain(|key, _| !selection.iter().any(|k| k == key));
                }

                overwrite_cmds(gen_cmds)?;
                println!("Removed {}", selection.join(", "));
            }
        }
    }
    Ok(())
}

fn insert_new_cmd(args: AddCmdUi, gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if gen_cmds.contains_key(&args.key) {
        return Err(anyhow!("A command by that key is in the database!"));
    }

    let gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
    let (key, cmd) = args.to_cmd()?;
    let aliases = cmd.aliases.clone();

    commit_cmd(gen_cmds, (key, cmd), aliases)
}
