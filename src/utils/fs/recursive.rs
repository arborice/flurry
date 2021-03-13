use crate::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use std::{fs::read_dir, path::PathBuf};

pub fn fetch_file_list<S: AsRef<str>>(
    path: &PathBuf,
    random: bool,
    filter: Option<S>,
    valid_exts: &[&str],
) -> Result<Vec<PathBuf>> {
    let mut file_list: Vec<PathBuf> = vec![];
    match filter {
        Some(filter) => filtered_recurse_dir(path, filter, &mut file_list, valid_exts),
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
            match path.extension() {
                Some(ext) => {
                    if valid_exts.iter().any(|e| *e == ext) {
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

fn filtered_recurse_dir<S: AsRef<str>>(
    dir_path: &PathBuf,
    filter: S,
    container: &mut Vec<PathBuf>,
    valid_exts: &[&str],
) -> Result<()> {
    for entry in read_dir(dir_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            match path.extension() {
                Some(ext) => {
                    if valid_exts.iter().any(|e| *e == ext)
                        && path.clone().to_string_lossy().contains(filter.as_ref())
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
