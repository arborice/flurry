use crate::prelude::*;

pub fn desktop_file_to_exec(mime_dfl: String) -> Result<String> {
    use std::fs::read;

    let desktop_file = mime_dfl.trim_end_matches("\n");
    let desktop_file_path = "/usr/share/applications/".to_owned() + desktop_file;
    let global_install = read(desktop_file_path)?;
    match find_exec_by_bytes(global_install) {
        Some(res) => Ok(res),
        _ => {
            let mut local_desktop_file_path = home();
            local_desktop_file_path.push(".local/share/applications/");
            local_desktop_file_path.push(desktop_file);
            let local_install = read(local_desktop_file_path)?;

            match find_exec_by_bytes(local_install) {
                Some(res) => Ok(res),
                _ => bail!("Unable to find exec cmd for {}", mime_dfl),
            }
        }
    }
}

fn find_exec_by_bytes(search_bytes: Vec<u8>) -> Option<String> {
    let (exec_pat, pat_0) = (b"Exec=", b'E');
    let last = search_bytes.len();
    let (mut start, mut end) = (last - 5, last);

    while start > 0 {
        if pat_0 == search_bytes[start] && exec_pat == &search_bytes[start..end] {
            let wsp = &search_bytes[start..]
                .iter()
                .position(|c| c.is_ascii_whitespace())?
                + start;
            return String::from_utf8(search_bytes[end..wsp].to_vec()).ok();
        }
        start -= 1;
        end -= 1;
    }
    None
}
