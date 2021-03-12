pub mod browser;
pub mod macros;
pub mod media;
pub mod os;
pub mod traits;

#[allow(dead_code)]
fn user_is_root() -> bool {
    if let Some(user) = std::env::var_os("USER") {
        user == "root"
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn ensure_root() {
    if !user_is_root() {
        crate::sudoku!("You must be superuser to use this command");
    }
}
