use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Archive, Deserialize, Serialize)]
pub struct GeneratedCommands {
	pub commands: Option<HashMap<String, GeneratedCommand>>,
}

#[derive(Clone, Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum PermissionsKind {
	Any,
	Group,
	Root,
	User,
}

impl AsRef<str> for PermissionsKind {
	fn as_ref(&self) -> &str {
		match self {
			PermissionsKind::Any => "any",
			PermissionsKind::Group => "group",
			PermissionsKind::Root => "root",
			PermissionsKind::User => "user",
		}
	}
}

impl From<bool> for PermissionsKind {
	fn from(switch: bool) -> PermissionsKind {
		if switch {
			PermissionsKind::Root
		} else {
			PermissionsKind::Any
		}
	}
}

impl Default for PermissionsKind {
	fn default() -> PermissionsKind {
		PermissionsKind::Any
	}
}

#[derive(Clone, Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum EncoderKind {
	RegEx(String),
	Raw(String),
	None,
	Url,
}

impl Default for EncoderKind {
	fn default() -> EncoderKind {
		EncoderKind::None
	}
}

#[derive(Clone, Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum ScanDirKind {
	Recursive,
	Depth(u8),
	None,
}

impl Default for ScanDirKind {
	fn default() -> ScanDirKind {
		ScanDirKind::None
	}
}

impl From<bool> for ScanDirKind {
	fn from(switch: bool) -> ScanDirKind {
		if switch {
			ScanDirKind::Recursive
		} else {
			ScanDirKind::None
		}
	}
}

#[derive(Clone, Debug, Default, PartialEq, Archive, Deserialize, Serialize)]
pub struct GeneratedCommand {
	pub bin: String,
	pub dfl_args: Option<Vec<String>>,
	pub aliases: Option<Vec<String>,
	pub encoder: EncoderKind,
	pub permissions: PermissionsKind,
	pub query_which: bool,
	pub scan_dir: ScanDirKind,
}

impl GeneratedCommand {
	pub fn change_alias_at(&mut self, index: usize, new_alias: &str) {
		if let Some(ref mut aliases) = self.aliases {
			aliases[index] = new_alias.into();
		}
	}

	pub fn toggle_permissions(&mut self) {
		use PermissionsKind::*;
		self.permissions = match self.permissions {
			Any => Group,
			Group => User,
			User => Root,
			Root => Any,
		}
	}

	pub fn set_scan_dir_depth(&mut self, depth: u8) {
		self.scan_dir = ScanDirKind::Depth(depth)
	}

	pub fn toggle_scan_dir_kind(&mut self) {
		use ScanDirKind::*;
		self.scan_dir = match self.scan_dir {
			Recursive => None,
			None => Recursive,
			_ => None,
		}
	}

	pub fn toggle_query_which(&mut self) {
		self.query_which = !self.query_which
	}
}
