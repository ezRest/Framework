use std::env;
use std::path::Path;

pub fn dir_exists(route: &str) {
    let dir = &format!("{}{}{}", env::current_dir().unwrap().display(), "/src", route);

    if !Path::new(dir).exists() {
        panic!("Base directory of mount point '/src{}' does not exists", route);
        // Err(fmt::Error) = write!("Base directory of mount point '/src{}' does not exists", route);
    }
}