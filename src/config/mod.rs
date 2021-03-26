pub mod get;
pub mod types;
pub mod write;

pub enum ConfigPath {
    Base,
    Commands,
    Pos,
}

impl ConfigPath {
    pub fn abs(&self) -> std::path::PathBuf {
        let mut path = crate::utils::os::home();
        path.push(match self {
            ConfigPath::Base => ".config/flurry",
            ConfigPath::Commands => ".config/flurry/commands.toml",
            ConfigPath::Pos => ".config/flurry/.pos",
        });
        path
    }
}
