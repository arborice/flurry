use crate::prelude::*;
use rkyv::{
	AlignedVec,
	ser::{Serializer, serializers::WriteSerializer},
	Serialize,
};
use std::fs::create_dir_all;

pub fn init_cmds_if_not_exists() -> Result<()> {
	if !ConfigPath::Base.abs().exists() {
		create_dir_all(ConfigPath::Base.abs())?;
		write(ConfigPath::Pos.abs(), "")?;
		write(
			ConfigPath::Commands.abs(),
			"# see <github link> for sample commands or create some with the cli\n",
		)?;
		Ok(())
	} else if !ConfigPath::Commands.abs().exists() {
		write(ConfigPath::Pos.abs(), "")?;
		write(
			ConfigPath::Commands.abs(),
			"# see <github link> for sample commands or create some with the cli\n",
		)?;
		Ok(())
	} else {
		bail!("Could not read from config dir. Insufficient permissions?")
	}
}

pub fn over_write_cmds(new_cmds: GeneratedCommands) -> Result<()> {
	let mut serializer = WriteSerializer::new(AlignedVec::new());
	let pos = serializer.serialize_value(&new_cmds)?;
	write(ConfigPath::Pos.abs(), pos.to_string())?;
	
	let buf = serializer.into_inner();
	write(ConfigPath::Commands.abs(), buf)?;
	Ok(())
}