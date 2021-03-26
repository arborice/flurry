use crate::{cli::types::AddCmd, config::write::*, prelude::*};
use rkyv::{core_impl::ArchivedOption, de::deserializers::AllocDeserializer, Deserialize};

pub fn insert_new_cmd(args: AddCmd, gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if let ArchivedOption::Some(ref cmds) = gen_cmds.commands {
        let key: &str = &args.key.as_ref();
        if cmds.contains_key(key) {
            bail!("A command by that key is in the database!");
        }

        let (key, cmd) = GeneratedCommand::from_args(args);
        let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
        if let Some(ref mut cmds) = gen_cmds.commands {
            cmds.insert(key, cmd);
        }
        overwrite_cmds(gen_cmds)
    } else {
        let mut cmds = HashMap::new();
        let (key, cmd) = GeneratedCommand::from_args(args);
        cmds.insert(key, cmd);
        overwrite_cmds(GeneratedCommands {
            commands: Some(cmds),
        })
    }
}
