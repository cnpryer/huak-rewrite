use std::path::Path;

///! This module implements operations Huak is capable of running. Huak can make
///! operations on Python projects, Python environments, etc.
///! NOTE: This module implements behavior meant for the CLI, so data about the project
///        or environment is only modified on the system. This behavior may change in
///        the future.
use crate::{error::HuakResult, Environment, Package, Project};

/// Activate a Python virtual environment.
pub fn activate_venv(env: Environment) -> HuakResult<()> {
    todo!()
}

/// Add a Python package as a dependency to a Python project.
pub fn add_project_dependency(
    dependency: Package,
    project: Project,
    env: Environment,
) -> HuakResult<()> {
    todo!()
}

/// Add a Python package as an optional dependency to a Python project.
pub fn add_project_optional_dependency(
    dependency: Package,
    group: &str,
    project: Project,
    env: Environment,
) -> HuakResult<()> {
    todo!()
}

/// Build the Python project as installable package.
pub fn build_project(project: Project, env: Environment) -> HuakResult<()> {
    todo!()
}

/// Clean the dist directory.
pub fn clean_project(project: Project) -> HuakResult<()> {
    todo!()
}

/// Format the Python project's source code.
pub fn format_project(project: Project) -> HuakResult<()> {
    todo!()
}

/// Initilize an existing Python project.
pub fn init_project(project: Project) -> HuakResult<()> {
    todo!()
}

/// Install a Python project's dependencies to an environment.
pub fn install_project_dependencies(project: Project, env: Environment) -> HuakResult<()> {
    todo!()
}

/// Install a group of a Python project's optional dependencies to an environment.
pub fn install_project_optional_dependencies(
    group: &str,
    project: Project,
    env: Environment,
) -> HuakResult<()> {
    todo!()
}

/// Lint a Python project's source code.
pub fn lint_project(project: Project, env: Environment) -> HuakResult<()> {
    todo!()
}

/// Fix any fixable lint issues found in the Python project's source code.
pub fn lint_fix_project(project: Project, env: Environment) -> HuakResult<()> {
    todo!()
}

/// Create a new Python project with all defaults.
pub fn create_new_default_project(root_path: impl AsRef<Path>) -> HuakResult<()> {
    todo!()
}

/// Create a new library-like Python project.
pub fn create_new_lib_project(root_path: impl AsRef<Path>) -> HuakResult<()> {
    todo!()
}

/// Create a new application-like Python project.
pub fn create_new_app_project(root_path: impl AsRef<Path>) -> HuakResult<()> {
    todo!()
}

/// Remove a dependency from a Python project.
pub fn remove_project_dependency(
    dependency_name: &str,
    project: Project,
    env: Environment,
) -> HuakResult<()> {
    todo!()
}

/// Remove a dependency from a Python project.
pub fn remove_project_optional_dependency(
    dependency_name: &str,
    group: &str,
    project: Project,
    env: Environment,
) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_str_with_context(command: &str, env: Environment) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_list_with_context(command: &Vec<&str>, env: Environment) -> HuakResult<()> {
    todo!()
}

/// Run a Python project's tests.
pub fn test_project(project: Project, env: Environment) -> HuakResult<()> {
    todo!()
}

/// Display the version of the Python project.
pub fn display_project_version(project: Project) -> HuakResult<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "currently untestable"]
    #[test]
    fn test_activate_venv() {
        todo!()
    }

    #[test]
    fn test_add_project_dependency() {
        todo!()
    }

    #[test]
    fn test_add_optional_project_dependency() {
        todo!()
    }

    #[test]
    fn test_build_project() {
        todo!()
    }

    #[test]
    fn test_clean_project() {
        todo!()
    }

    #[test]
    fn test_fmt_project() {
        todo!()
    }

    #[test]
    fn test_init_project() {
        todo!()
    }

    #[test]
    fn test_install_project_dependencies() {
        todo!()
    }

    #[test]
    fn test_install_project_optional_dependencies() {
        todo!()
    }

    #[test]
    fn test_lint_project() {
        todo!()
    }

    #[test]
    fn test_lint_fix_project() {
        todo!()
    }

    #[test]
    fn test_new_default_project() {
        todo!()
    }

    #[test]
    fn test_new_lib_project() {
        todo!()
    }

    #[test]
    fn test_new_app_project() {
        todo!()
    }

    #[ignore = "currently untestable"]
    #[test]
    fn test_publish_project() {
        todo!()
    }

    #[test]
    fn test_remove_project_dependency() {
        todo!()
    }

    #[test]
    fn test_remove_project_optional_dependency() {
        todo!()
    }

    #[test]
    fn test_run_command_with_context() {
        todo!()
    }

    #[test]
    fn test_test_project() {
        todo!()
    }

    #[test]
    fn test_display_project_version() {
        todo!()
    }
}
