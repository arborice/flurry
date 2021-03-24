use crate::{
    cli::types::RmCmd,
    config::{types::*, write::over_write_cmds},
    prelude::*,
};

pub fn try_rm_cmd(RmCmd { key, .. }: RmCmd, mut gen_cmds: GeneratedCommands) -> Result<()> {
    if let Some(ref mut cmds) = gen_cmds.commands {
        if cmds.contains_key(&key) {
            cmds.retain(|k, _| k != &key);
            return over_write_cmds(gen_cmds);
        }
    }

    bail!("No command found by that key")
}
