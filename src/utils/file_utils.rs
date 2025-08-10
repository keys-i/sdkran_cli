use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

/// Checks whether the given path exists and is a regular file.
///
/// # Examples
///
/// ```rust
/// use std::io;
/// use std::path::Path;
/// use tempfile::NamedTempFile;
/// use sdkran::utils::file_utils::check_file_exists;
///
/// # fn main() -> io::Result<()> {
/// let file = NamedTempFile::new()?;
/// let path = file.path();
///
/// // Should succeed
/// let validated = check_file_exists(path)?;
/// assert_eq!(validated, path.to_path_buf());
///
/// // Should fail for a non-existent file
/// let bad_path = Path::new("does_not_exist.txt");
/// let err = check_file_exists(bad_path).unwrap_err();
/// assert_eq!(err.kind(), io::ErrorKind::NotFound);
/// # Ok(())
/// # }
/// ```
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

/// Reads the contents of a file, trimming whitespace, and returns an error
/// if the file is empty.
///
/// # Examples
///
/// ```rust
/// use std::io::{self, Write};
/// use tempfile::NamedTempFile;
/// use sdkran::utils::file_utils::read_file_content;
///
/// # fn main() -> io::Result<()> {
/// // Write some data to a temp file
/// let mut file = NamedTempFile::new()?;
/// writeln!(file, "5.9.0")?;
///
/// // Read it back
/// let version = read_file_content(file.path())?;
/// assert_eq!(version, "5.9.0");
///
/// // Empty file should return InvalidData error
/// let empty_file = NamedTempFile::new()?;
/// let err = read_file_content(empty_file.path()).unwrap_err();
/// assert_eq!(err.kind(), io::ErrorKind::InvalidData);
/// # Ok(())
/// # }
/// ```
pub fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        Err(Error::new(ErrorKind::InvalidData, "File is empty"))
    } else {
        Ok(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{ErrorKind, Write},
        path::Path,
    };
    use tempfile::{NamedTempFile, TempDir};

    use crate::utils::file_utils::{check_file_exists, read_file_content};

    #[test]
    fn returns_ok_for_existing_file() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        let got = check_file_exists(path).expect("should be Ok for existing file");
        assert_eq!(got, path.to_path_buf());
    }

    #[test]
    fn returns_err_not_found_for_missing_file() {
        let missing = Path::new("this_file_should_not_exist_123456.txt");
        let err = check_file_exists(missing).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn returns_err_not_found_for_directory() {
        let dir = TempDir::new().unwrap();
        let err = check_file_exists(dir.path()).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn accepts_both_path_and_pathbuf() {
        let file = NamedTempFile::new().unwrap();

        // &Path
        let ok1 = check_file_exists(file.path()).unwrap();
        assert_eq!(ok1, file.path().to_path_buf());

        // PathBuf
        let ok2 = check_file_exists(file.path().to_path_buf()).unwrap();
        assert_eq!(ok2, file.path().to_path_buf());
    }

    #[test]
    fn should_read_content_from_file() {
        let expected_version = "5.9.0";
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(expected_version.as_bytes()).unwrap();

        let maybe_ver = read_file_content(file.path());
        assert_eq!(maybe_ver.as_deref().ok(), Some(expected_version));
    }

    #[test]
    fn should_fail_reading_file_content_from_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let maybe_ver = read_file_content(path);
        assert_eq!(
            maybe_ver.as_ref().map_err(|e| e.kind()),
            Err(ErrorKind::InvalidData)
        );
    }
}
