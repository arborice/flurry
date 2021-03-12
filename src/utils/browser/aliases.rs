use crate::utils::browser::bin::WebBrowser;
use once_cell::sync::Lazy;
use tinyvec::{array_vec, ArrayVec};

type Aliases = [&'static str; 4];
type LazyAliases = Lazy<ArrayVec<Aliases>>;

pub static BRAVE_ALIASES: LazyAliases =
    Lazy::new(|| array_vec!(Aliases => "br", "brave", "brave-browser"));

pub static FIREFOX_ALIASES: LazyAliases =
    Lazy::new(|| array_vec!(Aliases => "ff", "firefox", "firefox-esr", "firefox-nightly"));

pub static VIVALDI_ALIASES: LazyAliases =
    Lazy::new(|| array_vec!(Aliases => "vi", "vivaldi", "vivaldi-stable", "vivaldi-nightly"));

pub static BROWSER_ALIASES: Lazy<ArrayVec<[(&'static str, WebBrowser); 12]>> = Lazy::new(|| {
    let mut aliases = array_vec!([(&'static str, WebBrowser); 12]);

    for b in BRAVE_ALIASES.iter() {
        aliases.push((*b, WebBrowser::Brave));
    }

    for f in FIREFOX_ALIASES.iter() {
        aliases.push((*f, WebBrowser::Firefox));
    }

    for v in VIVALDI_ALIASES.iter() {
        aliases.push((*v, WebBrowser::Vivaldi));
    }

    aliases
});
