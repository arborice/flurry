use crate::{cli::types::SetCmd, config::write::*, prelude::*};
use rkyv::{de::deserializers::AllocDeserializer, Deserialize};

pub fn edit_cmd(args: SetCmd, gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    let SetCmd {
        target,
        bin,
        alias,
        permissions,
        scan_dir_depth,
        query_which,
        ext_filter,
        file_type_filter,
        encoder,
        args,
        append_args,
    } = args;

    if !gen_cmds.contains_key(&target) {
        return Err(anyhow!("{} does not exist", target));
    }

    if gen_cmds.is_alias(&target) {
        return Err(anyhow!(
            "{} is an alias, only commands can be edited",
            target
        ));
    }

    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;

    if let Some(ref new_alias) = alias {
        match &mut gen_cmds.aliases {
            Some(ref mut aliases) => {
                aliases.insert(new_alias.clone(), target.clone());
            }
            None => {
                let mut aliases = HashMap::new();
                aliases.insert(new_alias.clone(), target.clone());
                gen_cmds.aliases.replace(aliases);
            }
        }
    }

    if let Some(ref mut cmds) = gen_cmds.commands {
        cmds.get_mut(&target).and_then(|command| {
            if let Some(new_bin) = bin {
                command.bin = new_bin;
            }
            if let Some(new_alias) = alias {
                match &mut command.aliases {
                    Some(ref mut aliases) => {
                        if aliases.len() >= 4 {
                            eprintln!("only four aliases are allowed per command");
                        } else {
                            aliases.push(new_alias);
                        }
                    }
                    None => {
                        command.aliases.replace(vec![new_alias]);
                    }
                }
            }
            if let Some(new_permissions) = permissions {
                command.permissions = new_permissions;
            }
            if let Some(new_scan_dir_depth) = scan_dir_depth {
                command.scan_dir = new_scan_dir_depth;
            }
            if let Some(query_which) = query_which {
                command.query_which = query_which;
            }
            if let Some(mut new_ext_filter) = ext_filter {
                match &mut command.filter {
                    FiltersKind::None => command.filter = FiltersKind::One(new_ext_filter),
                    FiltersKind::One(filter) => match filter {
                        FilterKind::Exts(ref mut exts) => {
                            if let FilterKind::Exts(mut new_exts) = new_ext_filter {
                                exts.append(&mut new_exts);
                            }
                        }
                        _ => {
                            command.filter = FiltersKind::Many(vec![filter.clone(), new_ext_filter])
                        }
                    },
                    FiltersKind::Many(ref mut filters) => {
                        if let Some(pos) = filters.iter().position(|f| {
                            if let FilterKind::Exts(_) = f {
                                true
                            } else {
                                false
                            }
                        }) {
                            if let FilterKind::Exts(ref mut existing_exts) =
                                unsafe { filters.get_unchecked_mut(pos) }
                            {
                                if let FilterKind::Exts(ref mut new_exts) = new_ext_filter {
                                    existing_exts.append(new_exts);
                                }
                            }
                        } else {
                            filters.push(new_ext_filter);
                        }
                    }
                }
            }

            if let Some(new_file_type_filter) = file_type_filter {
                match &mut command.filter {
                    FiltersKind::None => command.filter = FiltersKind::One(new_file_type_filter),
                    FiltersKind::One(filter) => match filter {
                        FilterKind::FileType(_) => {
                            command.filter = FiltersKind::One(new_file_type_filter);
                        }
                        _ => {
                            command.filter =
                                FiltersKind::Many(vec![filter.clone(), new_file_type_filter]);
                        }
                    },
                    FiltersKind::Many(ref mut filters) => {
                        if let Some(pos) = filters.iter().position(|f| {
                            if let FilterKind::FileType(_) = f {
                                true
                            } else {
                                false
                            }
                        }) {
                            unsafe {
                                *filters.get_unchecked_mut(pos) = new_file_type_filter;
                            }
                        } else {
                            filters.push(new_file_type_filter);
                        }
                    }
                }
            }

            if let Some(new_encoder) = encoder {
                command.encoder.replace(new_encoder);
            }

            if let Some(mut new_args) = args {
                if let Some(ref mut dfl_args) = command.dfl_args {
                    if append_args {
                        dfl_args.append(&mut new_args);
                    } else {
                        *dfl_args = new_args;
                    }
                } else {
                    command.dfl_args.replace(new_args);
                }
            }

            Some(command)
        });
        overwrite_cmds(gen_cmds)?;
    }
    Ok(())
}
