use crate::{prelude::*, utils::os::linux::desktop_file_to_exec};
use std::path::PathBuf;

#[derive(Clone, PartialEq)]
pub enum Player {
    Audio,
    Image,
    Video,
}

impl Default for Player {
    fn default() -> Player {
        Player::Video
    }
}

impl AsRef<str> for Player {
    fn as_ref(&self) -> &str {
        match self {
            Player::Audio => "audio",
            Player::Image => "image",
            Player::Video => "video",
        }
    }
}

impl<'bin> Program<'bin> for Player {
    type Bin = BinKind<'bin>;

    fn get_bin(&self) -> Self::Bin {
        let desktop_file = self.try_query_mime().seppuku(None);
        BinKind::Owned(desktop_file_to_exec(desktop_file).seppuku(Some(
            "Unable to find handler for this media type. Please set an override in your config.",
        )))
    }
}

impl ProgramExec<'_, '_> for Player {
    type Args = Vec<PathBuf>;

    fn try_exec_override(&self, media_files: Self::Args, cfg: &GlobalConfig) -> Result<()> {
        if let Some(media_players) = &cfg.media_players {
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

    pub fn from_str(query: &str) -> Self {
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
