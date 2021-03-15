pub mod types;
pub mod write;

pub enum ConfigPath {
    Base,
    Config,
    Commands,
}

impl ConfigPath {
    pub fn abs(&self) -> std::path::PathBuf {
        let mut path = crate::utils::os::home();
        path.push(match self {
            ConfigPath::Base => ".config/flurry",
            ConfigPath::Config => ".config/flurry/config.toml",
            ConfigPath::Commands => ".config/flurry/commands.toml",
        });
        path
    }
}
