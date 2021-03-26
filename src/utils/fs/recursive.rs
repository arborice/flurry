use crate::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub enum FilterKind<'ext, S: AsRef<str>> {
    Exts(&'ext [S]),
    Raw(&'ext S),
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
        FilterKind::Exts(ext_filter) => ext_filtered_recurse_dir(path, &mut file_list, ext_filter),
        FilterKind::Raw(pat) => raw_filtered_recurse_dir(path, pat, &mut file_list),
        FilterKind::Regex(regex) => regex_filtered_recurse_dir(path, regex, &mut file_list),
        FilterKind::None => recurse_dir(path, &mut file_list),
    }?;

    if random {
        let mut rand_range = thread_rng();
        file_list.shuffle(&mut rand_range);
    }

    Ok(file_list)
}

fn recurse_dir<P: AsRef<Path>>(dir_path: P, container: &mut Vec<PathBuf>) -> Result<()> {
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            container.push(path)
        } else {
            recurse_dir(&path, container)?;
        }
    }
    Ok(())
}

fn ext_filtered_recurse_dir<P: AsRef<Path>, S: AsRef<str>>(
    dir_path: P,
    container: &mut Vec<PathBuf>,
    valid_exts: &[S],
) -> Result<()> {
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            match path.extension() {
                Some(ext) => {
                    if valid_exts.iter().any(|e| e.as_ref() == ext) {
                        container.push(path);
                    }
                }
                None => continue,
            }
        } else {
            ext_filtered_recurse_dir(&path, container, valid_exts)?;
        }
    }
    Ok(())
}

fn raw_filtered_recurse_dir<P: AsRef<Path>, S: AsRef<str>>(
    dir_path: P,
    filter: S,
    container: &mut Vec<PathBuf>,
) -> Result<()> {
    let filter = filter.as_ref();
    let base_path_is_match = dir_path.as_ref().to_string_lossy().contains(filter);
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            if base_path_is_match || path.to_string_lossy().contains(filter) {
                container.push(path);
            }
        } else {
            raw_filtered_recurse_dir(&path, filter, container)?;
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
