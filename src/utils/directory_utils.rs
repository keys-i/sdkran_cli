use directories::UserDirs;
use std::{env, path::PathBuf};

use crate::utils::constants::{DEFAULT_SDKMAN_HOME, SDKMAN_DIR_ENV_VAR};

/// Attempts to determine the SDKMAN directory.
///
/// If the environment variable [`SDKMAN_DIR_ENV_VAR`] is set, its value is used.
/// Otherwise, falls back to [`fallback_sdkman_dir`].
///
/// # Examples
///
/// Set the environment variable and retrieve it:
/// ```
/// use std::{env, path::PathBuf};
/// use sdkran::utils::directory_utils::infer_sdkman_dir;
/// use sdkran::utils::constants::SDKMAN_DIR_ENV_VAR;
///
/// let temp_dir = tempfile::TempDir::new().unwrap();
/// unsafe {
///     env::set_var(SDKMAN_DIR_ENV_VAR, temp_dir.path());
/// }
///
/// let dir = infer_sdkman_dir().unwrap();
/// assert_eq!(dir, temp_dir.path().to_path_buf());
/// ```
///
/// Unset the variable to fall back:
/// ```
/// use std::env;
/// use sdkran::utils::directory_utils::{infer_sdkman_dir, fallback_sdkman_dir};
/// use sdkran::utils::constants::SDKMAN_DIR_ENV_VAR;
///
/// unsafe {
///     env::remove_var(SDKMAN_DIR_ENV_VAR);
/// }
/// let dir = infer_sdkman_dir().unwrap();
/// assert_eq!(dir, fallback_sdkman_dir());
/// ```
pub fn infer_sdkman_dir() -> Result<PathBuf, std::env::VarError> {
    env::var(SDKMAN_DIR_ENV_VAR)
        .map(PathBuf::from)
        .or_else(|_| Ok(fallback_sdkman_dir()))
}

/// Returns the default SDKMAN directory, based on the user's home directory.
///
/// # Examples
///
/// ```
/// use sdkran::utils::directory_utils::fallback_sdkman_dir;
/// use sdkran::utils::constants::DEFAULT_SDKMAN_HOME;
/// use directories::UserDirs;
///
/// let expected = UserDirs::new()
///     .unwrap()
///     .home_dir()
///     .join(DEFAULT_SDKMAN_HOME);
/// assert_eq!(fallback_sdkman_dir(), expected);
/// ```
#[doc(hidden)]
pub fn fallback_sdkman_dir() -> PathBuf {
    UserDirs::new()
        .map(|dir| dir.home_dir().join(DEFAULT_SDKMAN_HOME))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::{
        constants::{DEFAULT_SDKMAN_HOME, SDKMAN_DIR_ENV_VAR},
        directory_utils::{fallback_sdkman_dir, infer_sdkman_dir},
    };

    use directories::UserDirs;
    use std::env::{remove_var, set_var, var_os};
    use tempfile::TempDir;

    #[test]
    fn returns_home_join_default_when_env_is_normal() {
        let expected = UserDirs::new()
            .unwrap()
            .home_dir()
            .join(DEFAULT_SDKMAN_HOME);
        assert_eq!(fallback_sdkman_dir(), expected);
    }

    #[test]
    fn respects_overridden_home_dir() {
        #[cfg(windows)]
        const HOME_KEY: &str = "USERPROFILE";
        #[cfg(not(windows))]
        const HOME_KEY: &str = "HOME";

        let temp_home = tempfile::TempDir::new().unwrap();
        let temp_home_path = temp_home.path().to_path_buf();

        let old = var_os(HOME_KEY);
        unsafe {
            set_var(HOME_KEY, temp_home_path.as_os_str());
        }

        let expected = temp_home_path.join(DEFAULT_SDKMAN_HOME);
        assert_eq!(fallback_sdkman_dir(), expected);

        unsafe {
            match old {
                Some(v) => set_var(HOME_KEY, v),
                None => remove_var(HOME_KEY),
            }
        }
    }

    #[test]
    fn returns_path_from_env_var_when_set() {
        let temp_dir = TempDir::new().unwrap();
        let expected = temp_dir.path().to_path_buf();
        unsafe {
            set_var(SDKMAN_DIR_ENV_VAR, expected.as_os_str());
        }
        let result = infer_sdkman_dir().unwrap();
        assert_eq!(result, expected);

        // Cleanup
        unsafe {
            remove_var(SDKMAN_DIR_ENV_VAR);
        }
    }

    #[test]
    fn falls_back_to_user_home_when_env_not_set() {
        unsafe {
            remove_var(SDKMAN_DIR_ENV_VAR);
        }

        let result = infer_sdkman_dir().unwrap();
        let expected = UserDirs::new()
            .unwrap()
            .home_dir()
            .join(DEFAULT_SDKMAN_HOME);

        assert_eq!(result, expected);
    }
}
