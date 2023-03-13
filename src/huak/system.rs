use std::{collections::HashMap, ffi::OsString, path::PathBuf};

use pep440_rs::Version;

use crate::{error::HuakResult, shell::Shell};

/// A struct to contain useful platform data and objects.
pub struct Platform {
    /// The name of the platform.
    name: String,
    /// Absolute paths to each Python interpreter installed.
    python_paths: HashMap<Version, PathBuf>,
    /// A shell to use for the platform.
    shell: Shell,
}

impl Platform {
    /// Create a new platform.
    pub fn new() -> Platform {
        todo!()
    }

    /// Install a Python interpreter.
    pub fn install_python(&mut self, version_str: &str) -> HuakResult<()> {
        todo!()
    }

    /// Get the absolute path to a specific Python interpreter with a version &str.
    pub fn python_path(&self, version_str: &str) -> Option<&PathBuf> {
        todo!()
    }

    /// Get the absolute path to the latest version Python interpreter installed.
    pub fn python_path_latest(&self) -> Option<&PathBuf> {
        todo!()
    }
}

/// Get a vector of paths from the system PATH environment variable.
pub fn env_path_values() -> Vec<PathBuf> {
    std::env::split_paths(&env_path_string()).collect()
}

pub fn env_path_string() -> OsString {
    match std::env::var_os("PATH") {
        Some(val) => val,
        None => OsString::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_shell_command() {
        let platform = Platform::new();

        todo!()
    }
}
