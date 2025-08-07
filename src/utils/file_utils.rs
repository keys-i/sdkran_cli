use std::{fs, path::PathBuf};

pub fn check_file_exists(path: PathBuf) -> PathBuf {
    if path.exists() && path.is_file() {
        path
    } else {
        panic!("not a valid path: {}", path.to_str().unwrap())
    }
}

pub fn read_file_content(path: PathBuf) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
    .filter(|s| !s.trim().is_empty())
    .map(|s| s.trim().to_string())
}
