use crate::prelude::*;
use rkyv::core_impl::ArchivedOption;

pub fn list_cmds(rkvyd_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if let ArchivedOption::Some(ref cmds) = rkvyd_cmds.commands {
        let cmds_list = cmds.keys().fold(String::new(), |mut res, k| {
            res += k.as_str();
            res.push('\n');
            res
        });
        print!("{}", cmds_list);
    }
    Ok(())
}
