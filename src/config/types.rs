use rkyv::{core_impl::ArchivedOption, Archive, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Archive, Deserialize, Serialize)]
pub struct GeneratedCommands {
	pub commands: Option<HashMap<String, GeneratedCommand>>,
	pub aliases: Option<HashMap<String, String>>,
}

impl ArchivedGeneratedCommands {
	pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
		let key = key.as_ref();
		if let ArchivedOption::Some(commands) = &self.commands {
			if commands.contains_key(key) {
				return true;
			}
		}
		if let ArchivedOption::Some(aliases) = &self.aliases {
			return aliases.contains_key(key);
		}
		false
	}

	pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&ArchivedGeneratedCommand> {
		if let ArchivedOption::Some(commands) = &self.commands {
			let key = key.as_ref();
			return commands.get(key).or_else(|| {
				if let ArchivedOption::Some(aliases) = &self.aliases {
					aliases.get(key).and_then(|key| commands.get(key))
				} else {
					None
				}
			});
		}
		None
	}
}

#[derive(Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum PermissionsKind {
	Any,
	Group,
	Root,
	User,
}

use std::fmt::{Display, Formatter, Result as FmtResult};
impl Display for ArchivedPermissionsKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				ArchivedPermissionsKind::Any => "any",
				ArchivedPermissionsKind::Group => "group",
				ArchivedPermissionsKind::Root => "root",
				ArchivedPermissionsKind::User => "user",
			}
		)
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

#[derive(Debug, PartialEq, Eq, Archive, Deserialize, Serialize)]
pub enum FileTypeFilter {
	Dirs,
	Files,
}

impl PartialEq<&ArchivedFileTypeFilter> for FileTypeFilter {
	fn eq(&self, og: &&ArchivedFileTypeFilter) -> bool {
		match self {
			FileTypeFilter::Dirs => {
				if let ArchivedFileTypeFilter::Dirs = og {
					true
				} else {
					false
				}
			}
			FileTypeFilter::Files => {
				if let ArchivedFileTypeFilter::Files = og {
					true
				} else {
					false
				}
			}
		}
	}
}

#[derive(Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum FilterKind {
	Exts(Vec<String>),
	FileType(FileTypeFilter),
	RegEx(String),
	Raw(String),
	None,
}

#[derive(Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum EncoderKind {
	Url,
	None,
}

impl Default for FilterKind {
	fn default() -> FilterKind {
		FilterKind::None
	}
}

#[derive(Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum ScanDirKind {
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
			ScanDirKind::Depth(u8::MAX)
		} else {
			ScanDirKind::None
		}
	}
}

#[derive(Debug, Default, PartialEq, Archive, Deserialize, Serialize)]
pub struct GeneratedCommand {
	pub bin: String,
	pub dfl_args: Option<Vec<String>>,
	pub aliases: Option<Vec<String>>,
	pub filter: FilterKind,
	pub permissions: PermissionsKind,
	pub query_which: bool,
	pub scan_dir: ScanDirKind,
}

use crate::cli::types::AddCmd;
impl GeneratedCommand {
	pub fn from_args(
		AddCmd {
			aliases,
			args,
			bin,
			key,
			permissions,
			query_which,
			scan_dir,
			filter,
			..
		}: AddCmd,
	) -> (String, GeneratedCommand) {
		(
			key,
			GeneratedCommand {
				aliases,
				bin,
				permissions: permissions.into(),
				scan_dir: scan_dir.into(),
				query_which,
				dfl_args: if args.is_empty() { None } else { Some(args) },
				filter: match filter {
					Some(e) => FilterKind::RegEx(e),
					None => FilterKind::None,
				},
			},
		)
	}

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

	pub fn toggle_query_which(&mut self) {
		self.query_which = !self.query_which
	}
}
