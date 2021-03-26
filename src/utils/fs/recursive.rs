use crate::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub enum Filter<'ext, S: AsRef<str>> {
    FileType(&'ext ArchivedFileTypeFilter),
    Exts(&'ext [S]),
    Raw(&'ext S),
    Regex(regex::Regex),
    None,
}

pub fn fetch_file_list<P: AsRef<Path>, S: AsRef<str>>(
    path: P,
    mut depth: u8,
    random: bool,
    filter: &Filter<S>,
) -> Result<Vec<PathBuf>> {
    let mut file_list: Vec<PathBuf> = vec![];
    match filter {
        Filter::Exts(ext_filter) => {
            ext_filtered_recurse_dir(path, &mut depth, &mut file_list, ext_filter)
        }
        Filter::FileType(ty) => {
            file_type_filtered_recurse_dir(path, &mut depth, &mut file_list, ty)
        }
        Filter::Raw(pat) => raw_filtered_recurse_dir(path, &mut depth, pat, &mut file_list),
        Filter::Regex(regex) => regex_filtered_recurse_dir(path, &mut depth, regex, &mut file_list),
        Filter::None => recurse_dir(path, &mut depth, &mut file_list),
    }?;

    if random {
        let mut rand_range = thread_rng();
        file_list.shuffle(&mut rand_range);
    }

    Ok(file_list)
}

fn recurse_dir<P: AsRef<Path>>(
    dir_path: P,
    depth: &mut u8,
    container: &mut Vec<PathBuf>,
) -> Result<()> {
    if *depth > 0 {
        *depth -= 1;
        for entry in read_dir(dir_path)? {
            let path = entry?.path();
            if path.is_dir() {
                recurse_dir(&path, depth, container)?;
            }
            container.push(path);
        }
    }
    Ok(())
}

fn file_type_filtered_recurse_dir<P: AsRef<Path>>(
    dir_path: P,
    depth: &mut u8,
    container: &mut Vec<PathBuf>,
    file_type: &ArchivedFileTypeFilter,
) -> Result<()> {
    if *depth > 0 {
        *depth -= 1;
        for entry in read_dir(dir_path)? {
            let path = entry?.path();
            let is_dir = path.is_dir();
            if is_dir {
                file_type_filtered_recurse_dir(&path, depth, container, file_type)?;
            }
            if (FileTypeFilter::Dirs == file_type && is_dir)
                || (FileTypeFilter::Files == file_type && !is_dir)
            {
                container.push(path);
            }
        }
    }
    Ok(())
}

fn ext_filtered_recurse_dir<P: AsRef<Path>, S: AsRef<str>>(
    dir_path: P,
    depth: &mut u8,
    container: &mut Vec<PathBuf>,
    valid_exts: &[S],
) -> Result<()> {
    if *depth > 0 {
        *depth -= 1;
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
                ext_filtered_recurse_dir(&path, depth, container, valid_exts)?;
            }
        }
    }
    Ok(())
}

fn raw_filtered_recurse_dir<P: AsRef<Path>, S: AsRef<str>>(
    dir_path: P,
    depth: &mut u8,
    filter: S,
    container: &mut Vec<PathBuf>,
) -> Result<()> {
    if *depth > 0 {
        *depth -= 1;
        let filter = filter.as_ref();
        let base_path_is_match = dir_path.as_ref().to_string_lossy().contains(filter);
        for entry in read_dir(dir_path)? {
            let path = entry?.path();
            if !path.is_dir() {
                if base_path_is_match || path.to_string_lossy().contains(filter) {
                    container.push(path);
                }
            } else {
                raw_filtered_recurse_dir(&path, depth, filter, container)?;
            }
        }
    }
    Ok(())
}

fn regex_filtered_recurse_dir<P: AsRef<Path>>(
    dir_path: P,
    depth: &mut u8,
    filter: &regex::Regex,
    container: &mut Vec<PathBuf>,
) -> Result<()> {
    if *depth > 0 {
        *depth -= 1;
        let base_path_is_match = filter.is_match(dir_path.as_ref().to_string_lossy().as_ref());
        for entry in read_dir(dir_path)? {
            let path = entry?.path();
            if !path.is_dir() {
                if base_path_is_match || filter.is_match(path.to_string_lossy().as_ref()) {
                    container.push(path);
                }
            } else {
                regex_filtered_recurse_dir(&path, depth, filter, container)?;
            }
        }
    }
    Ok(())
}
