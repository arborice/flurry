use crate::{
    config::types::*,
    prelude::*,
    utils::browser::{aliases::*, run::*},
};
use std::path::PathBuf;

#[derive(strum::AsRefStr, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum WebBrowser {
    Brave,
    Firefox,
    Vivaldi,
    #[strum(to_string = "xdg-open")]
    OsDfl,
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

impl AliasedProgram<'_> for WebBrowser {
    type Alias = &'static str;
    type Aliases = tinyvec::ArrayVec<[Self::Alias; 4]>;

    fn aliases(&self) -> Self::Aliases {
        use WebBrowser::*;
        match self {
            Brave => *BRAVE_ALIASES,
            Firefox => *FIREFOX_ALIASES,
            Vivaldi => *VIVALDI_ALIASES,
            OsDfl => tinyvec::array_vec!(),
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
        for (b, bin) in BROWSER_ALIASES.iter() {
            if b.eq_ignore_ascii_case(query) {
                return Some(bin.clone());
            }
        }
        None
    }
}
