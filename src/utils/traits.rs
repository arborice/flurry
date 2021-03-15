use crate::prelude::*;
use std::{ffi::OsStr, path::PathBuf};

pub trait Seppuku<'msg, T>: Sized {
    type Message;
    fn seppuku(self, exit_message: Self::Message) -> T;
}

impl<'msg, T> Seppuku<'msg, T> for Option<T> {
    type Message = &'msg str;
    fn seppuku(self, exit_message: Self::Message) -> T {
        match self {
            Some(any) => any,
            None => crate::seppuku!(exit_message),
        }
    }
}

impl<'msg, T, E: std::fmt::Display> Seppuku<'msg, T> for Result<T, E> {
    type Message = Option<&'msg str>;
    fn seppuku(self, exit_message: Self::Message) -> T {
        match self {
            Ok(any) => any,
            Err(e) => match exit_message {
                Some(msg) => crate::seppuku!(msg),
                None => crate::seppuku!(e),
            },
        }
    }
}

pub trait Program<'bin> {
    type Bin: AsRef<OsStr> + 'bin;

    fn get_bin(&self) -> Self::Bin;
}

pub trait NotFound {
    fn not_found<S: std::fmt::Display>(message: S) -> ! {
        seppuku!(404 => f"404 - {}", message)
    }
}

impl<'prog, P: Program<'prog>> NotFound for P {}

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

pub enum BinKind<'bin> {
    Borrowed(&'bin str),
    Owned(String),
    Whiched(PathBuf),
}

impl AsRef<OsStr> for BinKind<'_> {
    fn as_ref(&self) -> &OsStr {
        match self {
            Self::Borrowed(bin) => bin.as_ref(),
            Self::Owned(bin) => bin.as_ref(),
            Self::Whiched(bin) => bin.as_ref(),
        }
    }
}
