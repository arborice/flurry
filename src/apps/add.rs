use crate::{cli::types::AddCmd, config::write::overwrite_cmds, prelude::*};

pub fn insert_new_cmd(args: AddCmd, mut gen_cmds: GeneratedCommands) -> Result<()> {
    if let Some(ref mut cmds) = gen_cmds.commands {
        if cmds.contains_key(&args.key) {
            seppuku!("A command by that key is in the database!");
        }

    	let (key, cmd) = GeneratedCommand::from_args(args);
    	cmds.insert(key, cmd);
    	overwrite_cmds(gen_cmds)?;
    }
    Ok(())
}
