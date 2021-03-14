use crate::{cli::argh::AddCmd, config::write::append_cmd, prelude::*};

pub fn insert_new_cmd(args: AddCmd, gen_cmds: GeneratedCommands) -> Result<()> {
    let AddCmd {
        command_type,
        mut key,
        target,
    } = args;

    key.make_ascii_lowercase();

    if let Some(cmds) = gen_cmds.commands {
        if cmds.iter().any(|cmd| key.eq_ignore_ascii_case(cmd.key)) {
            seppuku!("A command by that key is in the database!");
        }
        append_cmd(GeneratedCommand {
            key: &key,
            target: &target,
            command_type: command_type.unwrap_or_default(),
        })
    } else {
        append_cmd(GeneratedCommand {
            key: &key,
            target: &target,
            command_type: command_type.unwrap_or_default(),
        })
    }?;
    println!("Added {}", key);
    Ok(())
}
