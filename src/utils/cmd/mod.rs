use crate::prelude::*;
use std::{ffi::OsStr, path::PathBuf};

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
        fs::recursive::{fetch_file_list, Filter},
        os::ensure_root,
    },
};

use rkyv::core_impl::ArchivedOption;

impl ArchivedGeneratedCommand {
    fn get_bin(&self) -> BinKind {
        if !self.query_which {
            BinKind::Borrowed(self.bin.as_ref())
        } else {
            if let Ok(bin) = which::which(self.bin.as_ref()) {
                return BinKind::Whiched(bin);
            }

            if let ArchivedOption::Some(aliases) = &self.aliases {
                for alias in aliases.iter() {
                    if let Ok(bin) = which::which(alias.as_ref()) {
                        return BinKind::Whiched(bin);
                    }
                }
            }

            seppuku!("Could not find a path for any alias");
        }
    }

    pub fn try_exec(&self, GoCmd { args, random, .. }: &GoCmd) -> Result<()> {
        let ArchivedGeneratedCommand {
            dfl_args,
            permissions,
            filter,
            scan_dir,
            ..
        } = self;

        if let ArchivedPermissionsKind::Root = permissions {
            ensure_root();
        }

        let filter = match filter {
            ArchivedFilterKind::Exts(exts) => Filter::Exts(exts.as_slice()),
            ArchivedFilterKind::FileType(ty) => Filter::FileType(ty),
            ArchivedFilterKind::RegEx(pat) => Filter::Regex(regex::Regex::new(pat)?),
            ArchivedFilterKind::Raw(pat) => Filter::Raw(pat),
            _ => Filter::None,
        };

        if let ArchivedScanDirKind::Depth(depth) = scan_dir {
            let files_list = args.iter().fold(vec![], |mut list, a| {
                match fetch_file_list(a, *depth, *random, &filter) {
                    Ok(ref mut files) => list.append(files),
                    Err(e) => eprintln!("Error getting files for {}: {}", a, e),
                }
                list
            });

            match &dfl_args {
                ArchivedOption::Some(dfl) => {
                    run_cmd!(@ self.get_bin() => dfl.iter().map(|a| a.as_ref()), files_list)
                }
                ArchivedOption::None => run_cmd!(@ self.get_bin() => files_list),
            }?;
        } else {
            match &dfl_args {
                ArchivedOption::Some(dfl) => {
                    run_cmd!(@ self.get_bin() => dfl.iter().map(|a| a.as_ref()), args)
                }
                ArchivedOption::None => run_cmd!(@ self.get_bin() => args),
            }?;
        }

        Ok(())
    }
}
