use crate::prelude::*;
use rkyv::{
	archived_value,
	de::deserializers::AllocDeserializer,
	Deserialize,	
};
use std::{fs::{read, File}, io::Read};

pub fn cmds_db() -> Result<GeneratedCommands> {
	let pos_file = read(ConfigPath::Pos.abs())?;
	let pos: usize = std::str::from_utf8(pos_file)?.parse()?;

	let db = read(ConfigPath::Commands.abs())?;
	let archived = unsafe { archived_value::<GeneratedCommands>(db.as_slice(), pos) };
	Ok(archived)
}

pub fn cmds_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<GeneratedCommands> {
	let mut import_file = File::open(&path)?;
	let mut buf = [0;10];
	import_file.read(&mut buf)?;
	let pos: usize = std::str::from_utf8(&buf)?.parse()?;

	let db = read(path)?;
	let archived = unsafe { archived_value::<GeneratedCommands>(db.as_slice(), pos) };
	let cmds = archived.serialize(&mut AllocDeserializer)?;
	Ok(cmds)
}