use crate::{cli::types::GoCmd, config::types::*, prelude::*};

fn cmd_fallback(bin: &String) -> Result<std::path::PathBuf> {
    let found = which::which(bin)?;
    Ok(found)
}

pub fn dispatch_from_args(args: GoCmd, cmds: GeneratedCommands) -> Result<()> {
    match &args.program {
        Some(bin) => {
            if let Some(commands) = cmds.commands {
                for (key, val) in &commands {
                    if bin == key {
                        return val.try_exec(&args);
                    }
                }
            }
            let fallback = cmd_fallback(&bin)?;
            run_cmd!(@ fallback => &args.args)
        }
        None => bail!("No command with that trigger"),
    }
}
