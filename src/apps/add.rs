use crate::{
    cli::types::{AddCmd, AddUtil},
    config::write::{append_cmd, append_util},
    prelude::*,
};

pub fn insert_new_cmd(
    AddCmd {
        command_type,
        mut key,
        target,
    }: AddCmd,
    gen_cmds: GeneratedCommands,
) -> Result<()> {
    key.make_ascii_lowercase();

    if let Some(cmds) = gen_cmds.commands {
        if cmds.iter().any(|cmd| key.eq_ignore_ascii_case(cmd.key)) {
            seppuku!("A command by that key is in the database!");
        }
    }
    append_cmd(GeneratedCommand {
        key: &key,
        target: &target,
        command_type: command_type.unwrap_or(CommandType::Url),
    })?;
    println!("Added {}", key);
    Ok(())
}

pub fn create_new_util(args: AddUtil, gen_cmds: GeneratedCommands) -> Result<()> {
    use crate::utils::programs::generic::UtilFromArgs;

    if let Some(cmds) = gen_cmds.commands {
        if cmds
            .iter()
            .any(|cmd| args.key.eq_ignore_ascii_case(cmd.key))
        {
            seppuku!("A command by that key is in the database!");
        }
    }

    let new_util = UtilFromArgs::from_args(args);
    append_util(new_util)?;
    Ok(())
}
