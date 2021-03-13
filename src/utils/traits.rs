use crate::prelude::*;
use std::ffi::OsStr;

pub trait Program<'bin> {
    type Bin: AsRef<OsStr> + 'bin;

    fn get_bin(&self) -> Self::Bin;
    fn not_found<S: std::fmt::Display>(message: S) -> ! {
        seppuku!(404 => f"404 - {}", message)
    }
}

pub trait AliasedProgram<'alias, 'bin>: Program<'bin> {
    type Alias: AsRef<OsStr> + AsRef<str> + 'alias;
    type Aliases;

    fn aliases(&self) -> Self::Aliases;
    fn is_override<'a>(&self, over_ride: &'a Self::Alias) -> bool;
}

pub trait ProgramExec<'args, 'bin>: Program<'bin> {
    type Args: 'args;

    fn try_exec_override(&self, args: Self::Args, cfg: &GlobalConfig) -> Result<()>;
    fn try_exec_os_dfl<O: AsRef<OsStr>, I: IntoIterator<Item = O>>(&self, args: I) -> Result<()> {
        run_cmd!(OS => args)?;
        Ok(())
    }
}
