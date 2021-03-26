use crate::prelude::*;
use std::{
	fs::{read, File},
	io::Read,
	pin::Pin,
};

pub struct CmdsDb {
	bytes: Vec<u8>,
	pos: usize,
}

impl CmdsDb {
	pub fn from_cfg() -> Result<Self> {
		let pos_file = read(ConfigPath::Pos.abs())?;
		let pos: usize = std::str::from_utf8(&pos_file)?.parse()?;

		let bytes = read(ConfigPath::Commands.abs())?;
		Ok(CmdsDb { pos, bytes })
	}

	pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
		let mut import_file = File::open(&path)?;
		let mut buf = [0; 10];
		import_file.read(&mut buf)?;
		let pos: usize = std::str::from_utf8(&buf)?.parse()?;

		let bytes = read(path)?;
		Ok(CmdsDb { pos, bytes })
	}

	pub fn archive(&self) -> &ArchivedGeneratedCommands {
		unsafe { rkyv::archived_value::<GeneratedCommands>(self.bytes.as_slice(), self.pos) }
	}

	#[allow(dead_code)]
	pub fn archive_mut(&mut self) -> Pin<&mut ArchivedGeneratedCommands> {
		unsafe {
			rkyv::archived_value_mut::<GeneratedCommands>(
				Pin::new(self.bytes.as_mut_slice()),
				self.pos,
			)
		}
	}
}
