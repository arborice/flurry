use crate::prelude::*;

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

/// handles validation of cli input based on associated VALID const
pub trait Valid {
    const VALID: &'static [&'static str];
    fn is_valid(query: &str) -> bool {
        Self::VALID.contains(&query.trim().to_lowercase().as_str())
    }
}
