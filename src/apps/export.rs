use crate::prelude::*;

pub fn export_gen_cmds(path: Option<&str>) -> Result<()> {
	let output_file = path.unwrap_or("flurry_commands.toml");
	let existing_cmds_as_bytes = std::fs::read(ConfigPath::Commands.abs())?;
	write(output_file, existing_cmds_as_bytes)?;
	Ok(())
}
