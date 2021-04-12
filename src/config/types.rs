use crate::utils::traits::Valid;
use rkyv::{core_impl::ArchivedOption, Archive, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Archive, Deserialize, Serialize)]
pub struct GeneratedCommands {
    pub commands: Option<HashMap<String, GeneratedCommand>>,
    pub aliases: Option<HashMap<String, String>>,
}

impl GeneratedCommands {
    pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
        let key = key.as_ref();
        if let Some(ref commands) = self.commands {
            if commands.contains_key(key) {
                return true;
            }
        }
        if let Some(ref aliases) = self.aliases {
            return aliases.contains_key(key);
        }
        false
    }

    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&GeneratedCommand> {
        if let Some(ref commands) = self.commands {
            let key = key.as_ref();
            return commands.get(key).or_else(|| {
                if let Some(ref aliases) = self.aliases {
                    aliases.get(key).and_then(|key| commands.get(key))
                } else {
                    None
                }
            });
        }
        None
    }
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

    pub fn is_alias<S: AsRef<str>>(&self, key: S) -> bool {
        if !self.contains_key(&key) {
            return false;
        }
        if let ArchivedOption::Some(aliases) = &self.aliases {
            return aliases.contains_key(key.as_ref());
        }

        false
    }
}

#[derive(Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum PermissionsKind {
    Any,
    Group,
    Root,
    User,
}

impl Valid for PermissionsKind {
    const VALID: &'static [&'static str] = &["none", "any", "group", "root", "user"];
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

#[derive(Clone, Debug, PartialEq, Eq, Archive, Deserialize, Serialize)]
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

#[derive(Clone, Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum FilterKind {
    Exts(Vec<String>),
    FileType(FileTypeFilter),
    RegEx(String),
    Raw(String),
    None,
}

impl Default for FilterKind {
    fn default() -> FilterKind {
        FilterKind::None
    }
}

#[derive(Clone, Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum FiltersKind {
    One(FilterKind),
    Many(Vec<FilterKind>),
    None,
}

impl Default for FiltersKind {
    fn default() -> FiltersKind {
        FiltersKind::None
    }
}

#[derive(Debug, PartialEq, Archive, Deserialize, Serialize)]
pub enum EncoderKind {
    Json,
    Url,
    None,
}

impl Valid for EncoderKind {
    const VALID: &'static [&'static str] = &["none", "json", "url", "web"];
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
    pub encoder: Option<EncoderKind>,
    pub aliases: Option<Vec<String>>,
    pub filter: FiltersKind,
    pub permissions: PermissionsKind,
    pub query_which: bool,
    pub scan_dir: ScanDirKind,
}

impl Valid for GeneratedCommand {
    const VALID: &'static [&'static str] = &["y", "yes", "true", "n", "no", "false"];
}

use crate::cli::types::AddCmd;
impl GeneratedCommand {
    pub fn from_args(
        AddCmd {
            aliases,
            args,
            bin,
            encoder,
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
                encoder,
                permissions: permissions.into(),
                scan_dir: scan_dir.into(),
                query_which,
                dfl_args: if args.is_empty() { None } else { Some(args) },
                filter: match filter {
                    Some(e) => FiltersKind::One(FilterKind::RegEx(e)),
                    None => FiltersKind::None,
                },
            },
        )
    }
}
