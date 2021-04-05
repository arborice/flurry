use crate::{cli::types::AddCmd, config::write::*, prelude::*};
use rkyv::{de::deserializers::AllocDeserializer, Deserialize};

pub fn insert_new_cmd(args: AddCmd, gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if gen_cmds.contains_key(&args.key) {
        bail!("A command by that key is in the database!")
    }

    let gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
    let aliases = args.aliases.clone();
    let (key, cmd) = GeneratedCommand::from_args(args);

    commit_cmd(gen_cmds, (key, cmd), aliases)
}

pub fn commit_cmd(
    mut gen_cmds: GeneratedCommands,
    (key, cmd): (String, GeneratedCommand),
    aliases: Option<Vec<String>>,
) -> Result<()> {
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
