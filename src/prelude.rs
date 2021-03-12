// macros
pub use super::{info, run_cmd, sudoku};
// crate utils
pub use super::config::read::ConfigPath;
pub use super::utils::{browser::bin::WebBrowser, ensure_root, os::query_stdin, traits::*};

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

pub trait Sudoku<T>: Sized {
    type Message;
    fn sudoku(self, exit_message: Self::Message) -> T;
}

impl<T> Sudoku<T> for Option<T> {
    type Message = &'static str;
    fn sudoku(self, exit_message: Self::Message) -> T {
        match self {
            Some(any) => any,
            None => crate::sudoku!(exit_message),
        }
    }
}
impl<T, E: std::fmt::Display> Sudoku<T> for Result<T, E> {
    type Message = Option<&'static str>;
    fn sudoku(self, exit_message: Self::Message) -> T {
        match self {
            Ok(any) => any,
            Err(e) => match exit_message {
                Some(msg) => crate::sudoku!(msg),
                None => crate::sudoku!(e),
            },
        }
    }
}
