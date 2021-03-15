use crate::{cli::types::ExportCmd, prelude::*};

pub fn export_gen_cmds(ExportCmd { output_file }: ExportCmd) -> Result<()> {
	let existing_cmds_as_bytes = std::fs::read(ConfigPath::Commands.abs())?;
	write(output_file, existing_cmds_as_bytes)?;
	println!("Export success");
	Ok(())
}
