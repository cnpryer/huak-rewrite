use std::path::Path;

use crate::error::HuakResult;

/// Initialize a directory on a local system as a git repository.
pub fn init_git(dir_path: impl AsRef<Path>) -> HuakResult<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_git() {
        todo!()
    }
}