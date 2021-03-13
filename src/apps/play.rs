use crate::{
    prelude::*,
    tui::prelude::*,
    utils::{fs::recursive::fetch_file_list, programs::media::player::Player},
};
use std::path::PathBuf;

struct MediaOpts<'m> {
    random: bool,
    files_dir: PathBuf,
    filter: Option<&'m str>,
    media_dir: &'m str,
    media_player: Player,
}

fn unwrap_matches<'m>(matches: &'m clap::ArgMatches) -> Option<MediaOpts<'m>> {
    let media_dir = matches.value_of("directory")?;
    let random = matches.is_present("random");
    let filter = matches.value_of("filter");
    let media_player = Player::from_matches(matches);
    let files_dir = PathBuf::from(&media_dir);

    Some(MediaOpts {
        random,
        files_dir,
        filter,
        media_dir,
        media_player,
    })
}

pub fn exec_media_from_matches(
    matches: &clap::ArgMatches,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    let MediaOpts {
        random,
        files_dir,
        filter,
        media_dir,
        media_player,
    } = unwrap_matches(matches).ok_or(anyhow!("No query provided"))?;

    let media_files = if !matches.is_present("case-insensitive-filter") {
        fetch_file_list(&files_dir, random, filter, media_player.valid_exts())
    } else {
        fetch_file_list(
            &files_dir,
            random,
            filter.map(|f| f.to_lowercase()),
            media_player.valid_exts(),
        )
    }?;
    println!("Opening {}", media_dir);

    if let Some(config) = cfg {
        media_player.try_exec_override(media_files, &config)
    } else {
        run_cmd!(@ media_player.get_bin() => media_files)
    }
}

impl ListEntry for PathBuf {
    fn as_context(&self) -> &str {
        self.to_str().unwrap()
    }

    fn as_entry(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

fn tui_opts<'opts, F: FnMut(usize)>(callback: F) -> Result<TuiOpts<'opts, F>> {
    let input_handler = TuiInputHandler {
        select: array_vec!(Ec => EventCap::with_key(' '), EventCap::LeftClick),
        exit: array_vec!(Ec => EventCap::with_key('\n'), EventCap::with_key('q'), EventCap::ctrl_c()),
        ..Default::default()
    };
    let event_loop = Events::with_exit_triggers(&input_handler.exit);
    let non_halting_callback = TuiCallback::NonHalting(callback);

    let opts = TuiOpts::new(input_handler, event_loop, non_halting_callback)?
        .with_selection_highlighter(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        );
    Ok(opts)
}

pub fn interactive(matches: &clap::ArgMatches, cfg: Option<GlobalConfig>) -> Result<()> {
    let MediaOpts {
        random,
        files_dir,
        filter,
        media_player,
        ..
    } = unwrap_matches(matches).ok_or(anyhow!("No query provided"))?;

    let mut media_files = if !matches.is_present("case-insensitive-filter") {
        fetch_file_list(&files_dir, random, filter, media_player.valid_exts())
    } else {
        fetch_file_list(
            &files_dir,
            random,
            filter.map(|f| f.to_lowercase()),
            media_player.valid_exts(),
        )
    }?;
    let media_files_ref = RefCell::from(&mut media_files);

    let mut playlist = vec![];
    let opts = tui_opts(|index| {
        playlist.push(index);
    })?;

    let last_entered_char = render(opts, &media_files_ref)?;

    let mut i = 0;
    media_files.retain(|_| (playlist.contains(&i), i += 1).0);
    match last_entered_char {
        '\n' => {
            if let Some(config) = cfg {
                media_player.try_exec_override(media_files, &config)
            } else {
                run_cmd!(@ media_player.get_bin() => media_files)
            }
        }
        _ => Ok(()),
    }
}
