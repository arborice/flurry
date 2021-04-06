use crate::{
	cli::types::ImportCmd,
	config::{types::*, write::*},
	prelude::*,
	utils::os::query_stdin,
};
use rkyv::{
	core_impl::ArchivedOption,
	de::deserializers::AllocDeserializer,
	std_impl::{chd::ArchivedHashMap, ArchivedString},
	Deserialize,
};

fn duplicate_checker<'a>(
	existing_cmds: &ArchivedHashMap<ArchivedString, ArchivedGeneratedCommand>,
	import_cmds: &'a ArchivedHashMap<ArchivedString, ArchivedGeneratedCommand>,
) -> Option<Vec<(&'a ArchivedString, String)>> {
	let mut found_dups: Option<Vec<(&ArchivedString, String)>> = None;
	for (key, cmd) in import_cmds.iter() {
		if existing_cmds.contains_key(key) {
			if let Some(replacement_key) = query_stdin(format!(
				"Command `{}` exists, targeting `{}`.\nEnter a replacement command trigger or skip import.",
				key,
				cmd.bin,
				)) {
				match found_dups {
					Some(ref mut dups) => dups.push((key, replacement_key)),
					None => {
						found_dups.replace(vec![(key, replacement_key)]);
					}
				}
			}
		}
	}

	found_dups
}

fn insert_cmds(
	existing_db: &ArchivedGeneratedCommands,
	import_cmds: &ArchivedHashMap<ArchivedString, ArchivedGeneratedCommand>,
) -> Result<GeneratedCommands> {
	let mut existing_db = existing_db.deserialize(&mut AllocDeserializer)?;
	let mut import_cmds = import_cmds.deserialize(&mut AllocDeserializer)?;

	if let Some(ref mut existing_cmds) = existing_db.commands {
		existing_cmds.reserve(import_cmds.capacity());
		for (key, cmd) in import_cmds.drain() {
			existing_cmds.insert(key, cmd);
		}
	} else {
		existing_db.commands.replace(import_cmds);
	}

	Ok(existing_db)
}

fn insert_and_dedup_cmds(
	existing_db: &ArchivedGeneratedCommands,
	import_cmds: &ArchivedHashMap<ArchivedString, ArchivedGeneratedCommand>,
	mut dups: Vec<(&ArchivedString, String)>,
) -> Result<GeneratedCommands> {
	let mut existing_db = existing_db.deserialize(&mut AllocDeserializer)?;
	if let Some(ref mut existing_cmds) = existing_db.commands {
		existing_cmds.reserve(import_cmds.len());

		for (key, cmd) in import_cmds.iter() {
			let key = if let Some(i) = dups.iter().position(|(k, _)| &&key.as_ref() == k) {
				dups.remove(i).1
			} else {
				key.deserialize(&mut AllocDeserializer)?
			};

			existing_cmds.insert(key, cmd.deserialize(&mut AllocDeserializer)?);
		}
	}

	Ok(existing_db)
}

pub fn import_cmds_from_file(
	ImportCmd { file_path }: ImportCmd,
	existing_db: &ArchivedGeneratedCommands,
) -> Result<()> {
	let new_archive = CmdsDb::from_path(file_path)?;
	let new_cmds = new_archive.archive();

	if let ArchivedOption::Some(ref import_cmds) = new_cmds.commands {
		if let ArchivedOption::Some(ref existing_cmds) = existing_db.commands {
			if let Some(dups) = duplicate_checker(existing_cmds, import_cmds) {
				overwrite_cmds(insert_and_dedup_cmds(existing_db, import_cmds, dups)?)?;
			} else {
				overwrite_cmds(insert_cmds(existing_db, import_cmds)?)?;
			}
		} else {
			let mut existing_db = existing_db.deserialize(&mut AllocDeserializer)?;
			let imported_cmds = new_cmds.commands.deserialize(&mut AllocDeserializer)?;
			existing_db.commands = imported_cmds;
			overwrite_cmds(existing_db)?;
		}
	} else {
		seppuku!("No commands to import");
	}

	println!("Commands imported");
	Ok(())
}
