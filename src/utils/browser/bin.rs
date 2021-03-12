use crate::{
    config::types::*,
    prelude::*,
    utils::browser::{aliases::*, run::*},
};
use std::path::PathBuf;

#[derive(Clone)]
pub enum WebBrowser {
    Brave,
    Firefox,
    Vivaldi,
    OsDfl,
}

impl AsRef<str> for WebBrowser {
    fn as_ref(&self) -> &str {
        match self {
            WebBrowser::Brave => "brave",
            WebBrowser::Firefox => "firefox",
            WebBrowser::Vivaldi => "vivaldi",
            WebBrowser::OsDfl => "xdg-open",
        }
    }
}

impl Default for WebBrowser {
    fn default() -> WebBrowser {
        WebBrowser::OsDfl
    }
}

impl<'bin> Program<'bin> for WebBrowser {
    type Bin = PathBuf;

    fn get_bin(&self) -> Self::Bin {
        if let Self::OsDfl = self {
            return PathBuf::from("xdg-open");
        }

        for alias in self.aliases() {
            if let Ok(bin) = which::which(alias) {
                return bin;
            }
        }
        Self::not_found(format!("Unable to find binary for {}", self.as_ref()))
    }
}

impl<'bin> AliasedProgram<'bin> for WebBrowser {
    type Alias = &'static str;
    type Aliases = &'bin [&'static str];

    fn aliases(&self) -> Self::Aliases {
        match self {
            WebBrowser::Brave => BRAVE_ALIASES,
            WebBrowser::Firefox => FIREFOX_ALIASES,
            WebBrowser::Vivaldi => VIVALDI_ALIASES,
            WebBrowser::OsDfl => Self::not_found("OS default is a reserved type"),
        }
    }
}

impl ProgramExec<'_, '_> for WebBrowser {
    type Args = String;

    fn try_exec_override(&self, url: Self::Args) -> Result<()> {
        let config_file = ConfigPath::Config.try_fetch()?;
        let config: GlobalConfig = toml::from_str(&config_file)?;

        if let Some(overrides) = config.overrides {
            for ProgramOverride { cmd, args } in &overrides {
                if self.is_override(*cmd) {
                    if let Some(browser_override) = <Self>::try_from_str(*cmd) {
                        return match args {
                            Some(args) => web_query_with_browser_args(&browser_override, args, url),
                            None => web_query(&browser_override, url),
                        };
                    }
                }
            }
        }

        web_query(self, url)
    }
}

impl WebBrowser {
    pub fn from_matches(matches: &clap::ArgMatches) -> Self {
        match matches.value_of("program") {
            None => Self::default_from_config(),
            Some(browser) => <Self>::try_from_str(browser),
        }
        .unwrap_or_default()
    }

    pub fn default_from_config() -> Option<Self> {
        let config_file = ConfigPath::Config.try_fetch().ok()?;
        let config: GlobalConfig = toml::from_str(&config_file).ok()?;
        let dfl_browser = config.default_browser?;
        <Self>::try_from_str(dfl_browser)
    }

    fn try_from_str(query: &str) -> Option<Self> {
        for (browser_aliases, browser) in &[
            (BRAVE_ALIASES, WebBrowser::Brave),
            (FIREFOX_ALIASES, WebBrowser::Firefox),
            (VIVALDI_ALIASES, WebBrowser::Vivaldi),
        ] {
            for alias in *browser_aliases {
                if alias.eq_ignore_ascii_case(query) {
                    return Some(browser.clone());
                }
            }
        }
        None
    }
}
