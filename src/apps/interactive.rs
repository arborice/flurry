use crate::{
    config::{types::*, write::overwrite_cmds},
    prelude::*,
    tui::{prelude::*, runtime::*},
};
use rkyv::{de::deserializers::AllocDeserializer, Deserialize};

pub fn dispatch_interactive(gen_cmds: Pin<&mut ArchivedGeneratedCommands>) -> Result<()> {
    if gen_cmds.commands.is_none() {
        return Err(anyhow!("No commands yet!"));
    }

    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
    if let Some(ref mut commands) = gen_cmds.commands {
        let mut cmds_list: Vec<(String, GeneratedCommand)> = commands.clone().drain().collect();
        let cmds_ref = RefCell::from(&mut cmds_list);

        let term_opts =
            TuiOpts::default().with_selection_highlighter(Style::default().fg(Color::Blue));

        let mut app = StatefulCmdsTable::with_items(&cmds_ref);
        let exit_status = app.render(term_opts)?;

        match exit_status.last_requested_action {
            Some(TuiInputHandler::GO) => {
                if let Some(key) = exit_status.go_request {
                    if let Some(cmd) = commands.get(&key) {
                        match &cmd.dfl_args {
                            Some(args) => run_cmd!(@ &cmd.bin => args),
                            None => run_cmd!(@ &cmd.bin =>),
                        }?;
                        return Ok(());
                    }
                }
            }
            Some(TuiInputHandler::RM) => {
                let rm_selection = exit_status.rm_selection;
                commands.retain(|key, _| !rm_selection.iter().any(|k| k == key));
                return overwrite_cmds(gen_cmds);
            }
            _ => {}
        }
        println!("Goodbye!");
    }
    Ok(())
}
