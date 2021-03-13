use crate::{
    config::{types::*, write::over_write_cmds},
    prelude::*,
    tui::prelude::*,
};

pub fn try_rm_cmd(matches: &clap::ArgMatches, mut gen_cmds: GeneratedCommands) -> Result<()> {
    let key = matches
        .value_of("key")
        .ok_or(anyhow!("No key value provided"))?;

    if let Some(mut cmds) = gen_cmds.commands {
        for (i, cmd) in cmds.iter().enumerate() {
            if key.eq_ignore_ascii_case(cmd.key) {
                cmds.remove(i);
                gen_cmds.commands = Some(cmds);
                return over_write_cmds(gen_cmds);
            }
        }
    }

    bail!("No command found by that key")
}

pub fn interactive(mut gen_cmds: GeneratedCommands) -> Result<()> {
    if let Some(ref mut cmds_list) = gen_cmds.commands {
        let cmds_list = RefCell::from(cmds_list);

        let input_handler = TuiInputHandler::default();
        let event_loop = Events::with_exit_triggers(&input_handler.exit);
        let remover = TuiCallback::NonHalting(|index| {
            cmds_list.borrow_mut().remove(index);
        });

        let term_opts = TuiOpts::new(input_handler, event_loop, remover)?.with_popup(&PopupOpts {
            title: "Confirm Deletion",
            message: "Remove {{ ctx }}?",
        });
        match render(term_opts, &cmds_list)? {
            '\n' | ' ' => over_write_cmds(gen_cmds),
            _ => Ok(()),
        }
    } else {
        bail!("No commands yet!")
    }
}
