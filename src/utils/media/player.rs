use crate::{config::types::*, prelude::*, utils::os::desktop_file_to_exec};
use std::path::PathBuf;

#[derive(strum::AsRefStr, Clone)]
pub enum Player {
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "image")]
    Image,
    #[strum(serialize = "video")]
    Video,
}

impl Program<'_> for Player {
    type Bin = String;

    fn get_bin(&self) -> Self::Bin {
        let desktop_file = self.try_query_mime().sudoku(None);
        desktop_file_to_exec(desktop_file).sudoku(Some(
            "Unable to find handler for this media type. Please set an override in your config.",
        ))
    }
}

impl ProgramExec<'_, '_> for Player {
    type Args = Vec<PathBuf>;

    fn try_exec_override(&self, media_files: Self::Args) -> Result<()> {
        let config_file = ConfigPath::Config.try_fetch()?;
        let config: GlobalConfig = toml::from_str(&config_file)?;

        if let Some(media_players) = config.media_players {
            if let Some(MediaPlayerOverride { bin, args }) = media_players.get(self.as_ref()) {
                match args {
                    Some(args) => run_cmd!(@ bin => args, media_files),
                    None => run_cmd!(@ bin => media_files),
                }?;
                return Ok(());
            }
        }
        run_cmd!(@ self.get_bin() => media_files)?;
        Ok(())
    }
}

impl Player {
    pub fn from_matches(matches: &clap::ArgMatches) -> Self {
        if let Some(media_player) = matches.value_of("player") {
            return <Self>::from_str(media_player);
        }
        Self::not_found("No media type provided")
    }

    fn try_query_mime(&self) -> Result<String> {
        let mime_query = match self {
            Player::Audio => "audio/mp3",
            Player::Image => "image/jpeg",
            Player::Video => "video/mp4",
        };
        let mime = std::process::Command::new("xdg-mime")
            .args(&["query", "default", mime_query])
            .output()?
            .stdout;

        let desktop_file = String::from_utf8(mime)?;
        Ok(desktop_file)
    }

    fn from_str(query: &str) -> Self {
        match query {
            "audio" => Self::Audio,
            "image" => Self::Image,
            "video" => Self::Video,
            _ => Self::not_found(format!("Invalid media format: {}", query)),
        }
    }

    pub fn valid_exts(&self) -> &[&str] {
        match self {
            Self::Audio => &["flac", "mp3", "wav"],
            Self::Image => &["jpg", "jpeg", "png"],
            Self::Video => &["mkv", "mov", "mp4", "m4v", "vid", "wmv"],
        }
    }
}
