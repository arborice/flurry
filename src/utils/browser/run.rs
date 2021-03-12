use crate::prelude::*;
use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

pub fn web_query<U>(browser: &WebBrowser, url: U) -> Result<()>
where
    U: AsRef<OsStr>,
{
    Command::new(browser.get_bin())
        .arg(url)
        .stdout(Stdio::null())
        .spawn()?;
    Ok(())
}

pub fn web_query_with_browser_args<Arg, Args, U>(
    browser: &WebBrowser,
    args: Args,
    url: U,
) -> Result<()>
where
    Arg: AsRef<OsStr>,
    Args: IntoIterator<Item = Arg>,
    U: AsRef<OsStr>,
{
    Command::new(browser.get_bin())
        .args(args)
        .arg(url)
        .stdout(Stdio::null())
        .spawn()?;
    Ok(())
}
