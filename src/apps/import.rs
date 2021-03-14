use crate::{cli::argh::ImportCmd, config::types::*, prelude::*};

fn duplicate_checker<'cmds>(
	all_cmds: &mut Vec<GeneratedCommand<'cmds>>,
	import_cmds: &mut Vec<GeneratedCommand<'cmds>>,
	edit_config_after: &mut bool,
) {
	let prev_len = all_cmds.len();
	all_cmds.append(import_cmds);
	all_cmds.sort_unstable_by_key(|cmd| cmd.key);

	let mut dup_checker = all_cmds.clone();
	dup_checker.dedup_by(|a, b| a.key.eq_ignore_ascii_case(b.key));

	if dup_checker.len() < prev_len {
		let dedup_query = "Duplicates detected. (d)edup, (e)dit file, or (i)gnore?";
		match query_stdin(dedup_query) {
			Some(d) if d.eq_ignore_ascii_case("d") => *all_cmds = dup_checker,
			Some(e) if e.eq_ignore_ascii_case("e") => *edit_config_after = true,
			Some(i) if i.eq_ignore_ascii_case("i") => {}
			_ => seppuku!("Import canceled"),
		}
	}
}

pub fn import_cmds_from_file(ImportCmd { file_path }: ImportCmd) -> Result<()> {
	let config_path = ConfigPath::Commands.abs();

	let file_contents = read_to_string(file_path)?;
	let import_deser: GeneratedCommands = toml::from_str(&file_contents)?;
	let mut import_cmds = import_deser.commands.seppuku("No commands to import");

	let curr_file = read_to_string(&config_path)?;
	let mut curr_deser: GeneratedCommands = toml::from_str(&curr_file)?;
	let mut edit_config_after = false;

	if let Some(ref mut all_cmds) = curr_deser.commands {
		duplicate_checker(all_cmds, &mut import_cmds, &mut edit_config_after);
	} else {
		curr_deser.commands.replace(import_cmds);
	}

	let re_serialized = toml::to_string(&curr_deser)?;
	write(&config_path, re_serialized)?;

	if edit_config_after {
		println!("Opening commands file");
		run_cmd!(OS => &config_path)?;
	}

	println!("Commands imported");
	Ok(())
}
