pub mod linux;

use crate::prelude::*;

pub fn query_stdin(query: &str) -> Option<String> {
    use std::io::stdin;

    println!("{}", query);
    let mut buffer = String::new();
    if let Err(e) = stdin().read_line(&mut buffer) {
        seppuku!(1 => f"Error reading input: {}", e);
    }
    buffer = buffer.trim().to_owned();
    if buffer.is_empty() {
        None
    } else {
        Some(buffer)
    }
}

pub fn home() -> std::path::PathBuf {
    home::home_dir().expect("Unable to find user home")
}
