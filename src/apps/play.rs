use crate::{
    cli::argh::PlayCmd, prelude::*, tui::prelude::*, utils::fs::recursive::fetch_file_list,
};

impl ListEntry for std::path::PathBuf {
    fn as_context(&self) -> &str {
        self.to_str().unwrap()
    }

    fn as_entry(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

pub fn exec_media_from_args(
    PlayCmd {
        case_insensitive_filter,
        directory,
        player,
        random,
        filter,
        ..
    }: PlayCmd,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    let media_files = if !case_insensitive_filter {
        fetch_file_list(&directory, random, filter, player.valid_exts())
    } else {
        fetch_file_list(
            &directory,
            random,
            filter.map(|f| f.to_lowercase()),
            player.valid_exts(),
        )
    }?;
    println!("Opening {}", directory.display());

    if let Some(config) = cfg {
        player.try_exec_override(media_files, &config)
    } else {
        run_cmd!(@ player.get_bin() => media_files)
    }
}

fn tui_opts<'opts, F: FnMut(usize)>(callback: F) -> Result<TuiOpts<'opts, F>> {
    let input_handler = TuiInputHandler {
        select: array_vec!(Ec => EventCap::with_key(' '), EventCap::LeftClick),
        exit: array_vec!(Ec => EventCap::with_key('\n'), EventCap::with_key('q'), EventCap::ctrl_c()),
        ..Default::default()
    };
    let non_halting_callback = TuiCallback::NonHalting(callback);

    let opts = TuiOpts::new(input_handler, non_halting_callback).with_selection_highlighter(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::ITALIC),
    );
    Ok(opts)
}

pub fn interactive_play(
    PlayCmd {
        case_insensitive_filter,
        directory,
        player,
        random,
        filter,
        ..
    }: PlayCmd,
    cfg: Option<GlobalConfig>,
) -> Result<()> {
    let mut media_files = if !case_insensitive_filter {
        fetch_file_list(&directory, random, filter, player.valid_exts())
    } else {
        fetch_file_list(
            &directory,
            random,
            filter.map(|f| f.to_lowercase()),
            player.valid_exts(),
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
                player.try_exec_override(media_files, &config)
            } else {
                run_cmd!(@ player.get_bin() => media_files)
            }
        }
        _ => Ok(()),
    }
}