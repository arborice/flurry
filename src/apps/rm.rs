use crate::{
    cli::types::RmCmd,
    config::{types::*, write::overwrite_cmds},
    prelude::*,
};
use rkyv::{de::deserializers::AllocDeserializer, Deserialize};

pub fn try_rm_cmd(RmCmd { key, alias }: RmCmd, gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if !gen_cmds.contains_key(&key) {
        return Err(anyhow!("No command or alias by that key"));
    }

    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
    if !alias {
        if let Some(ref mut cmds) = gen_cmds.commands {
            match cmds.remove(&key) {
                Some(cmd) => {
                    if let Some(aliases) = cmd.aliases {
                        if let Some(ref mut db_aliases) = gen_cmds.aliases {
                            for alias in aliases {
                                db_aliases.remove(&alias);
                            }
                        }
                    }
                }
                None => {
                    return Err(anyhow!(
                        "{} is an alias, run command with the -a flag to remove",
                        key
                    ))
                }
            }
        }
    } else {
        if let Some(ref mut db_aliases) = gen_cmds.aliases {
            if db_aliases.remove(&key).is_none() {
                return Err(anyhow!(
                    "{} is not an alias, run command without the -a flag to remove",
                    key
                ));
            }
        }
    }
    println!("Removing {}", key);
    overwrite_cmds(gen_cmds)
}
