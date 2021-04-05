use crate::{cli::types::ListCmd, prelude::*};
use rkyv::core_impl::ArchivedOption;

pub fn list_cmds(ListCmd { aliases }: ListCmd, rkvyd_cmds: &ArchivedGeneratedCommands) {
    if let ArchivedOption::Some(ref cmds) = rkvyd_cmds.commands {
        let cmds_list = if !aliases {
            cmds.keys()
                .fold(String::with_capacity(cmds.len()), |mut list, k| {
                    list.push_str(k.as_str());
                    list.push('\n');
                    list
                })
        } else {
            cmds.iter()
                .fold(String::with_capacity(cmds.len()), |mut list, (k, cmd)| {
                    let cmd_aliases = match &cmd.aliases {
                        ArchivedOption::Some(a) => a.join(", "),
                        ArchivedOption::None => String::new(),
                    };

                    list.push_str(k.as_str());
                    list.push_str(&cmd_aliases);
                    list.push('\n');
                    list
                })
        };
        print!("{}", cmds_list);
    }
}
