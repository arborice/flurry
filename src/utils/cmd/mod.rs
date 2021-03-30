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
    utils::{fs::recursive::*, os::ensure_root},
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
        if let ArchivedScanDirKind::Depth(depth) = scan_dir {
            let files_list =
                args.iter()
                    .fold(Ok(vec![]), |mut res, a| -> Result<Vec<PathBuf>> {
                        if let Ok(ref mut list) = res {
                            match filter {
                                ArchivedFiltersKind::One(filter) => {
                                    let filter = match filter {
                                        ArchivedFilterKind::Exts(exts) => Filter::Exts(exts),
                                        ArchivedFilterKind::FileType(ty) => Filter::FileType(ty),
                                        ArchivedFilterKind::RegEx(pat) => {
                                            Filter::Regex(regex::Regex::new(pat)?)
                                        }
                                        ArchivedFilterKind::Raw(pat) => Filter::Raw(pat),
                                        _ => Filter::None,
                                    };

                                    list.append(&mut fetch_file_list(a, *depth, *random, &filter)?);
                                }
                                ArchivedFiltersKind::Many(rkyvd_filters) => {
                                    let mut filters = vec![];
                                    for filter in rkyvd_filters.iter() {
                                        filters.push(match filter {
                                            ArchivedFilterKind::Exts(exts) => Filter::Exts(exts),
                                            ArchivedFilterKind::FileType(ty) => {
                                                Filter::FileType(ty)
                                            }
                                            ArchivedFilterKind::RegEx(pat) => {
                                                Filter::Regex(regex::Regex::new(pat)?)
                                            }
                                            ArchivedFilterKind::Raw(pat) => Filter::Raw(pat),
                                            _ => Filter::None,
                                        })
                                    }

                                    list.append(&mut fetch_many_filtered_file_list(
                                        a, *depth, *random, filters,
                                    )?);
                                }
                                ArchivedFiltersKind::None => {
                                    list.append(&mut fetch_file_list::<&String, &str>(
                                        a,
                                        *depth,
                                        *random,
                                        &Filter::None,
                                    )?);
                                }
                            }
                        }
                        res
                    })?;

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
