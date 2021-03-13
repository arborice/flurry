use crate::prelude::*;

pub fn export_gen_cmds(matches: &clap::ArgMatches) -> Result<()> {
	let output_file = matches
		.value_of("output-file")
		.unwrap_or("flurry_commands.toml");
	let existing_cmds_as_bytes = std::fs::read(ConfigPath::Commands.abs())?;
	write(output_file, existing_cmds_as_bytes)?;
	println!("Export success");
	Ok(())
}
