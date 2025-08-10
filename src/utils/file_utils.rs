use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

pub fn check_file_exists<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
    let path_buf = path.as_ref().to_path_buf();
    if path_buf.exists() && path_buf.is_file() {
        Ok(path_buf)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Not a valid file path: {}", path_buf.display()),
        ))
    }
}

pub fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        Err(Error::new(ErrorKind::InvalidData, "File is empty"))
    } else {
        Ok(trimmed)
    }
}
