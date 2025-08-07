use directories::UserDirs;
use std::{env, path::PathBuf};

use sdkran::utils::constants::{DEFAULT_SDKMAN_HOME, SDKMAN_DIR_ENV_VAR};

pub fn infer_sdkman_dir() -> PathBuf {
    match env::var(SDKMAN_DIR_ENV_VAR) {
        Ok(s) => PathBuf::from(s),
        Err(_) => fallback_sdkman_dir(),
    }
}

fn fallback_sdkman_dir() -> PathBuf {
    UserDirs::new()
        .map(|dir| dir.home_dir().join(DEFAULT_SDKMAN_HOME))
        .unwrap()
}
