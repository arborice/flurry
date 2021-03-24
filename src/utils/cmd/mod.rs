use crate::prelude::*;
use std::{ffi::OsStr, path::PathBuf};

impl PartialEq<GeneratedCommand> for String {
    fn eq(&self, cfg_util: &GeneratedCommand) -> bool {
        cfg_util.is_override(&self.as_ref()) || self == &cfg_util.bin
    }
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
            encoder,
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
                encoder: match encoder {
                    Some(e) => EncoderKind::RegEx(e),
                    None => EncoderKind::None,
                },
            },
        )
    }
}

enum BinKind<'bin> {
    Borrowed(&'bin str),
    Whiched(PathBuf),
}

impl AsRef<OsStr> for BinKind<'_> {
    fn as_ref(&self) -> &OsStr {
        match self {
            Self::Borrowed(bin) => bin.as_ref(),
            Self::Whiched(bin) => bin.as_ref(),
        }
    }
}

use crate::{
    cli::types::GoCmd,
    utils::{
        fs::recursive::{fetch_file_list, FilterKind},
        os::ensure_root,
    },
};

impl GeneratedCommand {
    fn get_bin(&self) -> BinKind {
        if !self.query_which {
            BinKind::Borrowed(self.bin.as_ref())
        } else {
            if let Some(aliases) = &self.aliases {
                for alias in aliases.iter() {
                    if let Ok(bin) = which::which(alias) {
                        return BinKind::Whiched(bin);
                    }
                }
            }

            seppuku!("Could not find a path for any alias");
        }
    }

    fn is_override(&self, over_ride: &str) -> bool {
        if let Some(aliases) = &self.aliases {
        	aliases.iter().any(|a| a == over_ride)
        } else {
            false
        }
    }

    pub fn try_exec(&self, GoCmd { args, random, .. }: &GoCmd) -> Result<()> {
        let GeneratedCommand {
            dfl_args,
            permissions,
            encoder,
            scan_dir,
            ..
        } = self;

        if let PermissionsKind::Root = permissions {
            ensure_root();
        }

        if let ScanDirKind::Recursive = scan_dir {
            let filter = match encoder {
                EncoderKind::RegEx(pat) => FilterKind::Regex::<&str>(regex::Regex::new(pat)?),
                EncoderKind::None => FilterKind::None,
                _ => todo!(),
            };
            let files_list = args.iter().fold(vec![], |mut list, a| {
                match fetch_file_list(a, *random, &filter) {
                    Ok(ref mut files) => list.append(files),
                    Err(e) => eprintln!("Error getting files for {}: {}", a, e),
                }
                list
            });

            match dfl_args {
                Some(dfl) => run_cmd!(@ self.get_bin() => dfl, files_list),
                None => run_cmd!(@ self.get_bin() => files_list),
            }?;
        } else {
            match dfl_args {
                Some(dfl) => run_cmd!(@ self.get_bin() => dfl, args),
                None => run_cmd!(@ self.get_bin() => args),
            }?;
        }

        Ok(())
    }
}
