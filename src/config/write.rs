use crate::prelude::*;
use rkyv::{
	ser::{serializers::WriteSerializer, Serializer},
	AlignedVec,
};
use std::fs::{create_dir_all, write};

pub fn init_cmds_if_not_exists() -> Result<()> {
	if !ConfigPath::Base.abs().exists() {
		create_dir_all(ConfigPath::Base.abs())?;
		overwrite_cmds(GeneratedCommands::default())?;
	} else if !ConfigPath::Commands.abs().exists() {
		overwrite_cmds(GeneratedCommands::default())?;
	}
	Ok(())
}

pub fn overwrite_cmds(new_cmds: GeneratedCommands) -> Result<()> {
	let mut serializer = WriteSerializer::new(AlignedVec::new());
	let pos = serializer.serialize_value(&new_cmds)?;
	write(ConfigPath::Pos.abs(), pos.to_string())?;
	let buf = serializer.into_inner();
	write(ConfigPath::Commands.abs(), buf)?;
	Ok(())
}
