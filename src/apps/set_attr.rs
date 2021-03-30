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
    if let Some(ref mut cmds) = gen_cmds.commands {
        cmds.get_mut(&target).and_then(|command| {
            if let Some(bin) = bin {
                command.bin = bin;
            }
            if let Some(alias) = alias {
                if let Some(ref mut aliases) = command.aliases {
                    if aliases.len() >= 4 {
                        eprintln!("only four aliases are allowed per command");
                    } else {
                        aliases.push(alias);
                    }
                }
            }
            if let Some(permissions) = permissions {
                command.permissions = permissions;
            }
            if let Some(scan_dir) = scan_dir_depth {
                command.scan_dir = scan_dir;
            }
            if let Some(query_which) = query_which {
                command.query_which = query_which;
            }
            if let Some(mut ext_filter) = ext_filter {
                match &mut command.filter {
                    FiltersKind::None => command.filter = FiltersKind::One(ext_filter),
                    FiltersKind::One(filter) => match filter {
                        FilterKind::Exts(ref mut exts) => {
                            if let FilterKind::Exts(mut new_exts) = ext_filter {
                                exts.append(&mut new_exts);
                            }
                        }
                        _ => command.filter = FiltersKind::Many(vec![filter.clone(), ext_filter]),
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
                                if let FilterKind::Exts(ref mut new_exts) = ext_filter {
                                    existing_exts.append(new_exts);
                                }
                            }
                        } else {
                            filters.push(ext_filter);
                        }
                    }
                }
            }

            if let Some(file_type_filter) = file_type_filter {
                match &mut command.filter {
                    FiltersKind::None => command.filter = FiltersKind::One(file_type_filter),
                    FiltersKind::One(filter) => match filter {
                        FilterKind::FileType(_) => {
                            command.filter = FiltersKind::One(file_type_filter)
                        }
                        _ => {
                            command.filter =
                                FiltersKind::Many(vec![filter.clone(), file_type_filter])
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
                                *filters.get_unchecked_mut(pos) = file_type_filter;
                            }
                        } else {
                            filters.push(file_type_filter);
                        }
                    }
                }
            }

            if let Some(encoder) = encoder {
                command.encoder.replace(encoder);
            }

            if let Some(mut args) = args {
                if let Some(ref mut dfl_args) = command.dfl_args {
                    if append_args {
                        dfl_args.append(&mut args);
                    } else {
                        *dfl_args = args;
                    }
                } else {
                    command.dfl_args.replace(args);
                }
            }

            Some(command)
        });
        overwrite_cmds(gen_cmds)?;
    }
    Ok(())
}
