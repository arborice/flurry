use crate::{
    cli::argh::RmCmd,
    config::{types::*, write::over_write_cmds},
    prelude::*,
    tui::prelude::*,
};

pub fn try_rm_cmd(RmCmd { key, .. }: RmCmd, mut gen_cmds: GeneratedCommands) -> Result<()> {
    let key = key.seppuku("No command key provided!");
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

pub fn interactive_rm(mut gen_cmds: GeneratedCommands) -> Result<()> {
    if let Some(ref mut cmds_list) = gen_cmds.commands {
        let cmds_list = RefCell::from(cmds_list);

        let input_handler = TuiInputHandler::default();
        let remover = TuiCallback::NonHalting(|index| {
            cmds_list.borrow_mut().remove(index);
        });

        let term_opts = TuiOpts::new(input_handler, remover).with_popup(&PopupOpts {
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
