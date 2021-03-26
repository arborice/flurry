use crate::{
    cli::types::RmCmd,
    config::{types::*, write::overwrite_cmds},
    prelude::*,
};
use rkyv::{core_impl::ArchivedOption, de::deserializers::AllocDeserializer, Deserialize};

pub fn try_rm_cmd(
    RmCmd { key, .. }: RmCmd,
    gen_cmds: Pin<&mut ArchivedGeneratedCommands>,
) -> Result<()> {
    if let ArchivedOption::Some(ref cmds) = gen_cmds.commands {
        let key: &str = key.as_ref();
        if cmds.contains_key(key) {
            let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
            if let Some(ref mut cmds) = gen_cmds.commands {
                cmds.remove(key);
            }
            return overwrite_cmds(gen_cmds);
        }
    }

    bail!("No command found by that key")
}
