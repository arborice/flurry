use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct GenericUtil<'c> {
    pub bin: &'c str,
    #[serde(rename = "args")]
    pub dfl_args: Option<Vec<&'c str>>,
    pub aliases: Option<Vec<&'c str>>,
    pub sanitizer: Option<&'c str>,
    pub permissions: bool,
    pub query_which: bool,
    pub scan_dir: bool,
}

#[derive(Default, Serialize, PartialEq)]
pub struct UtilFromArgs {
    pub bin: String,
    #[serde(rename = "args")]
    pub dfl_args: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    pub sanitizer: Option<String>,
    pub permissions: bool,
    pub query_which: bool,
    pub scan_dir: bool,
}

impl PartialEq<GenericUtil<'_>> for String {
    fn eq(&self, cfg_util: &GenericUtil) -> bool {
        cfg_util.is_override(&self.as_ref()) || self == cfg_util.bin
    }
}

use crate::cli::types::AddUtil;
impl UtilFromArgs {
    pub fn from_args(
        AddUtil {
            aliases,
            args,
            bin,
            permissions,
            query_which,
            scan_dir,
            sanitizer,
            ..
        }: AddUtil,
    ) -> UtilFromArgs {
        UtilFromArgs {
            aliases,
            bin,
            permissions,
            scan_dir,
            query_which,
            dfl_args: if args.is_empty() { None } else { Some(args) },
            sanitizer,
        }
    }
}

impl<'util> Program<'util> for GenericUtil<'util> {
    type Bin = BinKind<'util>;

    fn get_bin(&self) -> Self::Bin {
        if !self.query_which {
            BinKind::Borrowed(self.bin)
        } else {
            if let Some(aliases) = &self.aliases {
                for alias in aliases {
                    if let Ok(bin) = which::which(alias) {
                        return BinKind::Whiched(bin);
                    }
                }
            }

            Self::not_found("Could not find a path for any alias");
        }
    }
}

impl<'util> AliasedProgram<'util, 'util> for GenericUtil<'util> {
    type Alias = &'util str;
    type Aliases = Option<Vec<&'util str>>;

    fn aliases(&self) -> Self::Aliases {
        panic!("Unwrap the borrowed aliases to avoid cloning")
    }

    fn is_override(&self, over_ride: &Self::Alias) -> bool {
        if let Some(aliases) = &self.aliases {
            aliases.contains(over_ride)
        } else {
            false
        }
    }
}

use crate::{
    cli::types::GoCmd,
    utils::{
        ensure_root,
        fs::recursive::{fetch_file_list, FilterKind},
    },
};

impl GenericUtil<'_> {
    pub fn try_exec(&self, GoCmd { args, random, .. }: &GoCmd) -> Result<()> {
        let GenericUtil {
            dfl_args,
            permissions,
            sanitizer,
            scan_dir,
            ..
        } = self;

        if *permissions {
            ensure_root();
        }

        if *scan_dir {
            let filter = match sanitizer {
                Some(pat) => FilterKind::Regex::<&str>(regex::Regex::new(pat)?),
                None => FilterKind::None,
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
