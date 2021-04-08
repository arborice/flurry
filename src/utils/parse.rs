use crate::config::types::{EncoderKind, FileTypeFilter, FilterKind, PermissionsKind, ScanDirKind};

pub fn parse_with_delim<S: AsRef<str>>(arg: S, delimiter: &str) -> Option<Vec<String>> {
    let split: Vec<String> = arg
        .as_ref()
        .split(delimiter)
        .filter_map(|a| {
            if a.is_empty() {
                None
            } else {
                Some(a.to_owned())
            }
        })
        .collect();

    if split.is_empty() {
        None
    } else {
        Some(split)
    }
}

pub fn aliases_from_arg(arg: &str) -> Result<Vec<String>, String> {
    let aliases: Vec<String> = arg
        .splitn(5, ",")
        .take(4)
        .map(|alias| alias.trim().to_lowercase())
        .collect();

    if !aliases.is_empty() {
        Ok(aliases)
    } else {
        Err("No aliases provided!".into())
    }
}

// TODO: implement Aliases struct with const generics
// struct Aliases<const NUM: usize>([String; NUM]);
// pub fn aliases_from_arg(arg: &str) -> Result<Aliases<4>, String> {
// let mut aliases = Aliases([String::new(), String::new(), String::new(), String::new()]);
// for (i, alias) in arg.splitn(5, ",").take(4).enumerate() {
// aliases.0[i] = alias.trim().to_lowercase();
// }
//
// if !aliases.is_empty() {
// Ok(aliases)
// } else {
// Err("No aliases provided!".into())
// }
// }

pub fn recursion_limit_from_arg(arg: &str) -> Result<ScanDirKind, String> {
    match arg {
        "max" | "recursive" => Ok(ScanDirKind::Depth(u8::MAX)),
        "none" => Ok(ScanDirKind::None),
        _ => match arg.parse::<u8>() {
            Ok(0) => Ok(ScanDirKind::None),
            Ok(any) => Ok(ScanDirKind::Depth(any)),
            _ => Err(format!("{} is not a valid depth", arg)),
        },
    }
}

pub fn encoder_from_arg(arg: &str) -> Result<EncoderKind, String> {
    match arg {
        "url" | "web" => Ok(EncoderKind::Url),
        "json" => Ok(EncoderKind::Json),
        "none" | "n" | "false" => Ok(EncoderKind::None),
        _ => Err(String::from("valid inputs are url, json")),
    }
}

pub fn permissions_from_arg(arg: &str) -> Result<PermissionsKind, String> {
    match arg.trim() {
        "group" => Ok(PermissionsKind::Group),
        "user" => Ok(PermissionsKind::User),
        "root" => Ok(PermissionsKind::Root),
        "any" | "dfl" | "none" => Ok(PermissionsKind::Any),
        _ => Err(String::from("valid inputs are group, user, root, any")),
    }
}

pub fn exts_filter_from_arg(arg: &str) -> Result<FilterKind, String> {
    args_from_arg(arg)
        .map(|exts| FilterKind::Exts(exts))
        .map_err(|_| "no filters provided!".into())
}

pub fn file_type_filter_from_arg(arg: &str) -> Result<FilterKind, String> {
    match arg.trim() {
        "d" | "dir" | "dirs" | "directory" | "directories" => {
            Ok(FilterKind::FileType(FileTypeFilter::Dirs))
        }
        "f" | "file" | "files" => Ok(FilterKind::FileType(FileTypeFilter::Files)),
        _ => Err(String::from("valid inputs are d, dir, f , file")),
    }
}

pub fn args_from_arg(arg: &str) -> Result<Vec<String>, String> {
    parse_with_delim(arg, " ").ok_or("no args provided!".into())
}
