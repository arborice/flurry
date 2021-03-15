use crate::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub enum FilterKind<'ext, S: AsRef<str>> {
    Exts(&'ext [&'ext str]),
    Dual {
        eq_filter: S,
        ext_filter: Option<&'ext [&'ext str]>,
    },
    Regex(regex::Regex),
    None,
}

pub fn fetch_file_list<P: AsRef<Path>, S: AsRef<str>>(
    path: P,
    random: bool,
    filter: &FilterKind<S>,
) -> Result<Vec<PathBuf>> {
    let mut file_list: Vec<PathBuf> = vec![];
    match filter {
        FilterKind::Exts(ext_filter) => recurse_dir(path, &mut file_list, &Some(ext_filter)),
        FilterKind::Dual {
            eq_filter,
            ext_filter,
        } => filtered_recurse_dir(path, eq_filter, &mut file_list, ext_filter),
        FilterKind::Regex(regex) => regex_filtered_recurse_dir(path, regex, &mut file_list),
        FilterKind::None => recurse_dir(path, &mut file_list, &None),
    }?;

    if random {
        let mut rand_range = thread_rng();
        file_list.shuffle(&mut rand_range);
    }

    Ok(file_list)
}

fn recurse_dir<P: AsRef<Path>>(
    dir_path: P,
    container: &mut Vec<PathBuf>,
    valid_exts: &Option<&[&str]>,
) -> Result<()> {
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            match valid_exts {
                Some(ext_filter) => match path.extension() {
                    Some(ext) => {
                        if ext_filter.iter().any(|e| *e == ext) {
                            container.push(path);
                        }
                    }
                    None => continue,
                },
                None => container.push(path),
            }
        } else {
            recurse_dir(&path, container, valid_exts)?;
        }
    }
    Ok(())
}

fn filtered_recurse_dir<P: AsRef<Path>, S: AsRef<str>>(
    dir_path: P,
    filter: S,
    container: &mut Vec<PathBuf>,
    valid_exts: &Option<&[&str]>,
) -> Result<()> {
    let filter = filter.as_ref();
    let base_path_is_match = dir_path.as_ref().to_string_lossy().contains(filter);
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            match valid_exts {
                Some(ext_filter) => match path.extension() {
                    Some(ext) => {
                        if ext_filter.iter().any(|e| *e == ext)
                            && (base_path_is_match || path.to_string_lossy().contains(filter))
                        {
                            container.push(path);
                        }
                    }
                    None => continue,
                },
                None => {
                    if base_path_is_match || path.to_string_lossy().contains(filter) {
                        container.push(path);
                    }
                }
            }
        } else {
            filtered_recurse_dir(&path, filter, container, valid_exts)?;
        }
    }
    Ok(())
}

fn regex_filtered_recurse_dir<P: AsRef<Path>>(
    dir_path: P,
    filter: &regex::Regex,
    container: &mut Vec<PathBuf>,
) -> Result<()> {
    let base_path_is_match = filter.is_match(dir_path.as_ref().to_string_lossy().as_ref());
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            if base_path_is_match || filter.is_match(path.to_string_lossy().as_ref()) {
                container.push(path);
            }
        } else {
            regex_filtered_recurse_dir(&path, filter, container)?;
        }
    }
    Ok(())
}
