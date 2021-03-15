mod aliases;
pub mod run;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
pub fn encode_url<S: AsRef<str>>(url: S) -> String {
    utf8_percent_encode(url.as_ref(), FRAGMENT).to_string()
}

use crate::{
    prelude::*,
    utils::programs::browser::{aliases::*, run::*},
};

#[derive(Clone, PartialEq)]
pub enum WebBrowser {
    Brave,
    Chrome,
    Edge,
    Firefox,
    Safari,
    Vivaldi,
    OsDfl,
}

impl AsRef<str> for WebBrowser {
    fn as_ref(&self) -> &str {
        match self {
            WebBrowser::Brave => "brave",
            WebBrowser::Chrome => "chrome",
            WebBrowser::Edge => "edge",
            WebBrowser::Firefox => "firefox",
            WebBrowser::Safari => "Safari",
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
    type Bin = BinKind<'bin>;

    fn get_bin(&self) -> Self::Bin {
        if let Self::OsDfl = self {
            return BinKind::Borrowed("xdg-open");
        }

        for alias in self.aliases() {
            if let Ok(bin) = which::which(alias) {
                return BinKind::Whiched(bin);
            }
        }
        Self::not_found(format!("Unable to find binary for {}", self.as_ref()))
    }
}

impl<'alias> AliasedProgram<'alias, '_> for WebBrowser {
    type Alias = &'alias str;
    type Aliases = &'alias [&'static str];

    fn aliases(&self) -> Self::Aliases {
        match self {
            WebBrowser::Brave => BRAVE_ALIASES,
            WebBrowser::Chrome => CHROME_ALIASES,
            WebBrowser::Edge => EDGE_ALIASES,
            WebBrowser::Firefox => FIREFOX_ALIASES,
            WebBrowser::Safari => SAFARI_ALIASES,
            WebBrowser::Vivaldi => VIVALDI_ALIASES,
            WebBrowser::OsDfl => Self::not_found("OS default is a reserved type"),
        }
    }

    fn is_override(&self, over_ride: &Self::Alias) -> bool {
        self.aliases()
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(over_ride))
    }
}

impl ProgramExec<'_, '_> for WebBrowser {
    type Args = String;

    fn try_exec_override(&self, url: Self::Args, cfg: &GlobalConfig) -> Result<()> {
        if let Some(overrides) = &cfg.overrides {
            for ProgramOverride { cmd, args } in overrides {
                if self.is_override(cmd) {
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
    pub fn default_from_config(cfg: &GlobalConfig) -> Option<Self> {
        let dfl_browser = cfg.default_browser?;
        <Self>::try_from_str(dfl_browser)
    }

    pub fn try_from_str(query: &str) -> Option<Self> {
        for (browser_aliases, browser) in &[
            (BRAVE_ALIASES, WebBrowser::Brave),
            (CHROME_ALIASES, WebBrowser::Chrome),
            (EDGE_ALIASES, WebBrowser::Edge),
            (FIREFOX_ALIASES, WebBrowser::Firefox),
            (SAFARI_ALIASES, WebBrowser::Safari),
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
