use crate::prelude::*;
use std::ffi::OsStr;

pub trait Program<'bin> {
    type Bin: AsRef<OsStr> + 'bin;

    fn get_bin(&self) -> Self::Bin;
    fn not_found<S: std::fmt::Display>(message: S) -> ! {
        seppuku!(404 => f"404 - {}", message)
    }
}

pub trait AliasedProgram<'bin>: Program<'bin> {
    type Alias: AsRef<OsStr> + AsRef<str> + 'bin;
    type Aliases: IntoIterator<Item = &'bin Self::Alias>;

    fn aliases(&self) -> Self::Aliases;
    fn is_override<'a>(&self, over_ride: &'a str) -> bool {
        for alias in self.aliases() {
            let alias_ref: &str = alias.as_ref();
            if alias_ref.eq_ignore_ascii_case(over_ride) {
                return true;
            }
        }
        false
    }
}

pub trait ProgramExec<'args, 'bin>: Program<'bin> {
    type Args: 'args;

    fn try_exec_override(&self, args: Self::Args) -> Result<()>;
    fn try_exec_os_dfl<O: AsRef<OsStr>, I: IntoIterator<Item = O>>(&self, args: I) -> Result<()> {
        run_cmd!(OS => args)?;
        Ok(())
    }
}
