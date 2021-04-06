use crate::{
    config::{types::*, write::overwrite_cmds},
    prelude::*,
    tui::prelude::*,
};
use rkyv::{de::deserializers::AllocDeserializer, Deserialize};

pub fn dispatch_interactive(gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if gen_cmds.commands.is_none() {
        return Err(anyhow!("No commands yet!"));
    }

    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
    let mut exit_status = None;
    let mut success_msg = String::new();

    if let Some(ref mut commands) = gen_cmds.commands {
        let cmds_ref = RefCell::from(commands);

        let mut app = StatefulCmdsTable::with_items(&cmds_ref)
            .with_header_style(Style::default().fg(Color::Blue))
            .with_rm_style(Style::default().fg(Color::Red));

        let status = app.render()?;
        if status.success {
            if !status.rm_selection.is_empty() {
                let ref selection = status.rm_selection;
                cmds_ref
                    .borrow_mut()
                    .retain(|key, _| !selection.iter().any(|k| k == key));
                success_msg += &format!("Removed {}", selection.join(", "));
            }

            exit_status.replace(status);
        }
    }

    if let Some(status) = exit_status {
        if let Some(key) = status.go_request {
            if let Some(cmd) = gen_cmds.get(&key) {
                match &cmd.dfl_args {
                    Some(args) => run_cmd!(@ &cmd.bin => args),
                    None => run_cmd!(@ &cmd.bin =>),
                }?;
            }
        }

        overwrite_cmds(gen_cmds)?;
        println!("{}", success_msg);
    }
    Ok(())
}
