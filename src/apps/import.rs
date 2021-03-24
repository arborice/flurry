use crate::{cli::types::ImportCmd, config::{types::*, read::*, write::*}, prelude::*};

fn duplicate_checker(
	existing_cmds: &mut HashMap<String, GeneratedCommand>,
	import_cmds: &mut HashMap<String, GeneratedCommand>,
) {
	existing_cmds.reserve(import_cmds.capacity());

	for (key, cmd) in import_cmds.drain() {
		if let Some(failed_import) = existing_cmds.insert(key, cmd) {
			println!("Command by same key exists -> {:?}", failed_import);
		}
	}
}

pub fn import_cmds_from_file(ImportCmd { file_path }: ImportCmd) -> Result<()> {
	let mut existing_cmds = cmds_db()?;
	let mut new_cmds = cmds_from_file(file_path)?;

	if let Some(ref mut import_cmds) = new_cmds.commands {
		if let Some(ref mut existing_cmds) = existing_cmds.commands {
			duplicate_checker(existing_cmds, import_cmds);
		} else {
			existing_cmds.commands = new_cmds.commands;
		}
	} else {
		seppuku!("No commands to import");
	}

	overwrite_cmds(existing_cmds)?;
	println!("Commands imported");
	Ok(())
}
