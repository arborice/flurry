use crate::{cli::types::GoCmd, config::types::*, prelude::*};
use rkyv::core_impl::ArchivedOption;

pub fn dispatch_from_args(args: GoCmd, cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if let ArchivedOption::Some(ref commands) = cmds.commands {
        let key: &str = args.command.as_ref();
        if let Some(cmd) = commands.get(key) {
            return cmd.try_exec(&args);
        }
    }
    bail!("No command found by that key")
}
