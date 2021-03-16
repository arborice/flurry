pub mod fs;
pub mod macros;
pub mod os;
pub mod programs;
pub mod traits;

fn user_is_root() -> bool {
    if let Some(user) = std::env::var_os("USER") {
        user == "root"
    } else {
        false
    }
}

pub fn ensure_root() {
    if !user_is_root() {
        crate::seppuku!("You must be superuser to use this command");
    }
}
