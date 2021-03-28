use crate::{cli::types::GoCmd, config::types::*, prelude::*};

pub fn dispatch_from_args(args: GoCmd, cmds: &ArchivedGeneratedCommands) -> Result<()> {
    cmds.get(&args.command)
        .ok_or(anyhow!("No command found by that key"))
        .and_then(|cmd| cmd.try_exec(&args))
}
