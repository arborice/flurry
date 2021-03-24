use crate::{
    config::{types::*, write::over_write_cmds},
    prelude::*,
    tui::{prelude::*, runtime::*},
};

pub fn dispatch_interactive(mut gen_cmds: GeneratedCommands) -> Result<()> {
    if let Some(ref mut commands) = gen_cmds.commands {
        let mut cmds_list: Vec<(String, GeneratedCommand)> = commands.clone().drain().collect();
        let cmds_ref = RefCell::from(&mut cmds_list);

        let rm_callback = Callback::Rm(CallbackOpts {
            halting: false,
            popup_opts: Some(
                PopupOpts::default()
                    .title("Confirm Deletion")
                    .message("Remove {{ ctx }}?")
                    .add_context(),
            ),
            callback: Box::new(remover),
        });

        let run_callback = Callback::Go(CallbackOpts {
            halting: true,
            popup_opts: None,
            callback: Box::new(runner),
        });

        let term_opts =
            TuiOpts::default().with_selection_highlighter(Style::default().fg(Color::Blue));

        let app = StatefulCmdsTable::with_items(&cmds_ref);
		let exit_status = app.render(&term_opts)?;

        match exit_status.last_requested_callback {
            Some(Callback::GO) => {
            	if let Some(key) = exit_status.go_request {
                	if let Some(cmd) = commands.get(&key) {
                    	match &cmd.dfl_args {
                    	    Some(args) => run_cmd!(@ &cmd.bin => args),
                    	    None => run_cmd!(@ &cmd.bin =>),
                    	}?;
                    	return Ok(());
                    }
                }
            },
            Some(Callback::RM) => {
                let rm_selection = exit_status.rm_selection;
                commands.retain(|key, _| !rm_selection.iter().any(|k| k == key));
                return over_write_cmds(gen_cmds);
            }
            _ => {}
        }
        println!("Goodbye!");
        Ok(())
    } else {
        bail!("No commands yet!")
    }
}
