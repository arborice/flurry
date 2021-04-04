use crate::{
    config::{types::*, write::overwrite_cmds},
    prelude::*,
    tui::{prelude::*, runtime::*},
};
use rkyv::{
    core_impl::ArchivedOption, de::deserializers::AllocDeserializer, std_impl::ArchivedString,
    Deserialize,
};

pub fn dispatch_interactive(gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if gen_cmds.commands.is_none() {
        return Err(anyhow!("No commands yet!"));
    }

    if let ArchivedOption::Some(ref commands) = gen_cmds.commands {
        let cmds_list: Vec<(&ArchivedString, &ArchivedGeneratedCommand)> =
            commands.iter().collect();
        let cmds_ref = RefCell::from(&cmds_list);

        let tui_opts =
            TuiOpts::default().with_selection_highlighter(Style::default().fg(Color::Blue));

        let mut app = StatefulCmdsTable::with_items(&cmds_ref);
        let exit_status = app.render(tui_opts)?;

        if exit_status.success {
            match exit_status.last_requested_action {
                Some(TuiInputHandler::ADD) => {
                    if let Some(cmd) = exit_status.new_cmd {
                        let key = cmd.key.clone();
                        match insert_new_cmd(cmd, gen_cmds) {
                            Ok(_) => {
                                println!("{} successfully added", key);
                                return Ok(());
                            }
                            Err(e) => bail!(e),
                        }
                    }
                }
                Some(TuiInputHandler::GO) => {
                    if let Some(key) = exit_status.go_request {
                        if let Some(cmd) = gen_cmds.get(&key) {
                            match &cmd.dfl_args {
                                ArchivedOption::Some(args) => {
                                    run_cmd!(@ cmd.bin.as_ref() => args.iter().map(|a| a.as_ref()))
                                }
                                ArchivedOption::None => run_cmd!(@ cmd.bin.as_ref() =>),
                            }?;
                            return Ok(());
                        }
                    }
                }
                Some(TuiInputHandler::RM) => {
                    let rm_selection = exit_status.rm_selection;
                    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
                    if let Some(ref mut cmds) = gen_cmds.commands {
                        cmds.retain(|key, _| !rm_selection.iter().any(|k| k == key));
                    }

                    match overwrite_cmds(gen_cmds) {
                        Ok(_) => {
                            println!("Removed {}", rm_selection.join(", "));
                            return Ok(());
                        }
                        Err(e) => bail!(e),
                    }
                }
                _ => {}
            }
            println!("Goodbye!");
        }
    }
    Ok(())
}

fn insert_new_cmd(args: table::AddCmdUi, gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if gen_cmds.contains_key(&args.key) {
        bail!("A command by that key is in the database!")
    }

    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
    let (key, cmd) = args.to_cmd()?;
    let aliases = cmd.aliases.clone();

    if let Some(ref mut rkyvd_cmds) = gen_cmds.commands {
        if let Some(failed_inserts) = if let Some(ref mut rkyvd_aliases) = gen_cmds.aliases {
            aliases.map(|mut list| {
                list.drain(..).fold(vec![], |mut failed, alias| {
                    if rkyvd_cmds.contains_key(&alias) || rkyvd_aliases.contains_key(&alias) {
                        failed.push(alias);
                    } else {
                        rkyvd_aliases.insert(alias, key.clone());
                    }
                    failed
                })
            })
        } else {
            let mut aliases_map = HashMap::new();
            let failed_inserts = aliases.map(|mut list| {
                list.drain(..).fold(vec![], |mut failed, alias| {
                    if rkyvd_cmds.contains_key(&alias) {
                        failed.push(alias);
                    } else {
                        aliases_map.insert(alias, key.clone());
                    }
                    failed
                })
            });
            gen_cmds.aliases.replace(aliases_map);
            failed_inserts
        } {
            if !failed_inserts.is_empty() {
                let has = if failed_inserts.len() > 1 {
                    "have"
                } else {
                    "has"
                };
                println!(
                    "{} already existed and {} not been inserted",
                    failed_inserts.join(", "),
                    has,
                );
            }
        }

        rkyvd_cmds.insert(key, cmd);
        overwrite_cmds(gen_cmds)
    } else {
        let aliases = aliases.map(|mut list| {
            list.drain(..).fold(HashMap::new(), |mut map, alias| {
                (map.insert(alias, key.clone()), map).1
            })
        });

        let mut cmds = HashMap::new();
        cmds.insert(key, cmd);

        overwrite_cmds(GeneratedCommands {
            commands: Some(cmds),
            aliases,
        })
    }
}
