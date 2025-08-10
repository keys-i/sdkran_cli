use directories::UserDirs;
use std::{env, path::PathBuf};

use crate::utils::constants::{DEFAULT_SDKMAN_HOME, SDKMAN_DIR_ENV_VAR};

pub fn infer_sdkman_dir() -> Result<PathBuf, std::env::VarError> {
    env::var(SDKMAN_DIR_ENV_VAR)
        .map(PathBuf::from)
        .or_else(|_| Ok(fallback_sdkman_dir()))
}

fn fallback_sdkman_dir() -> PathBuf {
    UserDirs::new()
        .map(|dir| dir.home_dir().join(DEFAULT_SDKMAN_HOME))
        .unwrap()
}

#[cfg(test)]
mod tests {}
