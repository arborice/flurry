use crate::{prelude::*, tui::prelude::*, utils::media::player::Player};
use rand::{seq::SliceRandom, thread_rng};
use std::{fs::read_dir, path::PathBuf};

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

pub fn exec_media_from_matches(matches: &clap::ArgMatches) -> Result<()> {
    let MediaOpts {
        random,
        files_dir,
        filter,
        media_dir,
        media_player,
    } = unwrap_matches(matches).ok_or(anyhow!("No query provided"))?;

    let media_files = fetch_file_list(
        &files_dir,
        random,
        filter.map(|f| f.to_lowercase()),
        media_player.valid_exts(),
    )?;
    println!("Opening {}", media_dir);

    media_player.try_exec_override(media_files)?;
    Ok(())
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

pub fn interactive(matches: &clap::ArgMatches) -> Result<()> {
    let MediaOpts {
        random,
        files_dir,
        filter,
        media_player,
        ..
    } = unwrap_matches(matches).ok_or(anyhow!("No query provided"))?;

    let mut media_files = fetch_file_list(
        &files_dir,
        random,
        filter.map(|f| f.to_lowercase()),
        media_player.valid_exts(),
    )?;
    let media_files_ref = RefCell::from(&mut media_files);

    let mut playlist = vec![];
    let opts = tui_opts(|index| {
        playlist.push(index);
    })?;

    let last_entered_char = render(opts, &media_files_ref)?;

    let mut i = 0;
    media_files.retain(|_| (playlist.contains(&i), i += 1).0);
    match last_entered_char {
        '\n' => media_player.try_exec_override(media_files),
        _ => Ok(()),
    }
}

fn fetch_file_list(
    path: &PathBuf,
    random: bool,
    filter: Option<String>,
    valid_exts: &[&str],
) -> Result<Vec<PathBuf>> {
    let mut file_list: Vec<PathBuf> = vec![];
    match filter {
        Some(filter) => filtered_recurse_dir(path, &filter, &mut file_list, valid_exts),
        None => recurse_dir(path, &mut file_list, valid_exts),
    }?;

    if random {
        let mut rand_range = thread_rng();
        file_list.shuffle(&mut rand_range);
    }

    Ok(file_list)
}

fn recurse_dir(
    dir_path: &PathBuf,
    container: &mut Vec<PathBuf>,
    valid_exts: &[&str],
) -> Result<()> {
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            match get_owned_ext(&path) {
                Some(ext) => {
                    if valid_exts.contains(&ext) {
                        container.push(path);
                    }
                }
                None => continue,
            }
        } else {
            recurse_dir(&path, container, valid_exts)?;
        }
    }
    Ok(())
}

fn get_owned_ext(path: &PathBuf) -> Option<&str> {
    path.extension()?.to_str()
}

fn filtered_recurse_dir(
    dir_path: &PathBuf,
    filter: &str,
    container: &mut Vec<PathBuf>,
    valid_exts: &[&str],
) -> Result<()> {
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            match get_owned_ext(&path) {
                Some(ext) => {
                    if valid_exts.contains(&ext) && path.clone().to_string_lossy().contains(filter)
                    {
                        container.push(path);
                    }
                }
                None => continue,
            }
        } else {
            recurse_dir(&path, container, valid_exts)?;
        }
    }
    Ok(())
}
