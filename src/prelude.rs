// macros
pub use super::{run_cmd, seppuku};
// crate utils
pub use super::config::{read::ConfigPath, types::GlobalConfig};
pub use super::utils::{
    ensure_root,
    os::query_stdin,
    programs::browser::{bin::WebBrowser, run::web_query},
    traits::*,
};

// std lib
pub use std::fs::{read_to_string, write};
pub use std::io::ErrorKind::NotFound as FileNotFound;

// third party
pub use anyhow::{anyhow, bail, Result};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

pub fn home() -> std::path::PathBuf {
    home::home_dir().expect("Unable to find user home")
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
pub fn encode_url<S: AsRef<str>>(url: S) -> String {
    utf8_percent_encode(url.as_ref(), FRAGMENT).to_string()
}

pub trait Seppuku<T>: Sized {
    type Message;
    fn seppuku(self, exit_message: Self::Message) -> T;
}

impl<T> Seppuku<T> for Option<T> {
    type Message = &'static str;
    fn seppuku(self, exit_message: Self::Message) -> T {
        match self {
            Some(any) => any,
            None => crate::seppuku!(exit_message),
        }
    }
}

impl<T, E: std::fmt::Display> Seppuku<T> for Result<T, E> {
    type Message = Option<&'static str>;
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
