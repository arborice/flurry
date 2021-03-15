use crate::{cli::types::ImportCmd, config::types::*, prelude::*};

fn duplicate_checker<'cmds>(
	all_cmds: &mut Vec<GeneratedCommand<'cmds>>,
	import_cmds: &mut Vec<GeneratedCommand<'cmds>>,
	edit_config_after: &mut bool,
) {
	all_cmds.append(import_cmds);
	all_cmds.sort_unstable_by_key(|cmd| cmd.key);

	let mut prev_key = "";
	if all_cmds
		.clone()
		.iter()
		.any(|cmd| (cmd.key.eq_ignore_ascii_case(prev_key), prev_key = cmd.key).0)
	{
		let dedup_query = "Duplicates detected. (d)edup, (e)dit file, or (i)gnore?";
		match query_stdin(dedup_query) {
			Some(d) if d.eq_ignore_ascii_case("d") => {
				all_cmds.dedup_by(|a, b| a.key.eq_ignore_ascii_case(b.key))
			}
			Some(e) if e.eq_ignore_ascii_case("e") => *edit_config_after = true,
			Some(i) if i.eq_ignore_ascii_case("i") => {}
			_ => seppuku!("Import canceled"),
		}
	}
}

pub fn import_cmds_from_file(ImportCmd { file_path }: ImportCmd) -> Result<()> {
	let config_path = ConfigPath::Commands.abs();

	let import_file = read_to_string(file_path)?;
	let new_cmds: GeneratedCommands = toml::from_str(&import_file)?;
	let mut import_cmds = new_cmds.commands.seppuku("No commands to import");

	let cmd_file = read_to_string(&config_path)?;
	let mut existing_cmds: GeneratedCommands = toml::from_str(&cmd_file)?;
	let mut edit_config_after = false;

	if let Some(ref mut all_cmds) = existing_cmds.commands {
		duplicate_checker(all_cmds, &mut import_cmds, &mut edit_config_after);
	} else {
		existing_cmds.commands.replace(import_cmds);
	}

	let updated_cmds_file = toml::to_string(&existing_cmds)?;
	write(&config_path, updated_cmds_file)?;

	if edit_config_after {
		println!("Opening commands file");
		run_cmd!(OS => &config_path)?;
	}

	println!("Commands imported");
	Ok(())
}
