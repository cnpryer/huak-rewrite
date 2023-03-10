///! This module implements various operations that can be executed against
///! valid workspaces.
use crate::{error::HuakResult, Environment, Package, Project, Workspace};
use std::path::Path;

/// Activate a Python virtual environment.
pub fn activate_venv(env: Environment) -> HuakResult<()> {
    todo!()
}

/// Add Python packages as a dependencies to a Python project.
pub fn add_project_dependencies<'a>(
    workspace: &mut Workspace<'a>,
    dependencies: &[Package],
) -> HuakResult<Workspace<'a>> {
    todo!()
}

/// Add Python packages as optional dependencies to a Python project.
pub fn add_project_optional_dependencies<'a>(
    workspace: &mut Workspace<'a>,
    dependencies: &[Package],
    groups: &[&str],
) -> HuakResult<Workspace<'a>> {
    todo!()
}

/// Build the Python project as installable package.
pub fn build_project<'a>(workspace: &Workspace<'a>) -> HuakResult<()> {
    todo!()
}

/// Clean the dist directory.
pub fn clean_project<'a>(workspace: &Workspace<'a>) -> HuakResult<()> {
    todo!()
}

/// Format the Python project's source code.
pub fn format_project<'a>(workspace: &Workspace<'a>) -> HuakResult<()> {
    todo!()
}

/// Initilize an existing Python project.
pub fn init_project<'a>(workspace: &mut Workspace<'a>) -> HuakResult<Workspace<'a>> {
    todo!()
}

/// Install a Python project's dependencies to an environment.
pub fn install_project_dependencies<'a>(
    workspace: &mut Workspace<'a>,
) -> HuakResult<Workspace<'a>> {
    todo!()
}

/// Install groups of a Python project's optional dependencies to an environment.
pub fn install_project_optional_dependencies<'a>(
    workspace: &mut Workspace<'a>,
    groups: &[&str],
) -> HuakResult<Workspace<'a>> {
    todo!()
}

/// Lint a Python project's source code.
pub fn lint_project<'a>(workspace: &Workspace<'a>) -> HuakResult<()> {
    todo!()
}

/// Fix any fixable lint issues found in the Python project's source code.
pub fn fix_project_lints<'a>(workspace: &Workspace<'a>) -> HuakResult<()> {
    todo!()
}

/// Create a new Python project with all defaults on the system.
pub fn create_new_default_project(root_path: impl AsRef<Path>) -> HuakResult<()> {
    todo!()
}

/// Create a new library-like Python project on the system.
pub fn create_new_lib_project(root_path: impl AsRef<Path>) -> HuakResult<()> {
    todo!()
}

/// Create a new application-like Python project on the system.
pub fn create_new_app_project(root_path: impl AsRef<Path>) -> HuakResult<()> {
    todo!()
}

/// Remove a dependency from a Python project.
pub fn remove_project_dependencies<'a>(
    workspace: &mut Workspace<'a>,
    dependency_names: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Remove a dependency from a Python project.
pub fn remove_project_optional_dependencies<'a>(
    workspace: &Workspace<'a>,
    dependency_names: &[&str],
    groups: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_str_with_context<'a>(
    workspace: &Workspace<'a>,
    command: &str,
) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_list_with_context<'a>(
    workspace: &Workspace<'a>,
    command: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Run a Python project's tests.
pub fn test_project<'a>(workspace: &Workspace<'a>) -> HuakResult<()> {
    todo!()
}

/// Display the version of the Python project.
pub fn display_project_version<'a>(workspace: &Workspace<'a>) -> HuakResult<()> {
    todo!()
}

/// NOTE: Operations are meant to be executed on projects and environments.
///       See https://github.com/cnpryer/huak/issues/123
///       To run some of these tests a .venv must be available at the project's root.
#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use tempfile::tempdir;

    use crate::{test_resources_dir_path, PyProjectTomlWrapper};

    use super::*;

    #[ignore = "currently untestable"]
    #[test]
    fn test_activate_venv() {
        todo!()
    }

    #[test]
    fn test_add_project_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let deps = [Package::from_str("ruff").unwrap()];
        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        ws = add_project_dependencies(&mut ws, &deps).unwrap();
        let project = ws.project();
        let env = ws.environment("default");
        let ser_toml =
            PyProjectTomlWrapper::from_path(dir.join("mock-project").join("pyproject.toml"))
                .unwrap();

        assert!(env.find_site_packages_package("ruff").is_some());
        assert!(deps
            .iter()
            .all(|item| project.dependencies().contains(item)));
        assert!(deps
            .iter()
            .map(|item| item)
            .all(|item| ser_toml.dependencies().contains(&item.display())));
        assert!(deps.iter().map(|item| item).all(|item| project
            .pyproject_toml_wrapper()
            .dependencies()
            .contains(&item.display())));
    }

    #[test]
    fn test_add_optional_project_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let deps = [Package::from_str("ruff").unwrap()];
        let groups = ["test"];
        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        ws = add_project_optional_dependencies(&mut ws, &deps, &groups).unwrap();
        let project = ws.project();
        let env = ws.environment("default");
        let ser_toml =
            PyProjectTomlWrapper::from_path(dir.join("mock-project").join("pyproject.toml"))
                .unwrap();

        assert!(env.find_site_packages_package("ruff").is_some());
        assert!(deps
            .iter()
            .all(|item| project.optional_dependencey_group("test").contains(item)));
        assert!(deps.iter().map(|item| item).all(|item| ser_toml
            .optional_dependencey_group("test")
            .contains(&item.display())));
        assert!(deps.iter().map(|item| item).all(|item| project
            .pyproject_toml_wrapper()
            .optional_dependencey_group("test")
            .contains(&item.display())));
    }

    #[test]
    fn test_build_project() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let env = Environment::venv(".venv").unwrap();
        ws.add_environment("default", env).unwrap();

        build_project(&ws).unwrap();
    }

    #[test]
    fn test_clean_project() {
        todo!()
    }

    #[test]
    fn test_fmt_project() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let fmt_filepath = ws
            .project()
            .root()
            .join("src")
            .join("mock_project")
            .join("fmt_me.py");
        let pre_fmt_str = r#"""
def fn( ):
    pass"#;
        std::fs::write(&fmt_filepath, pre_fmt_str).unwrap();

        format_project(&ws).unwrap();

        let post_fmt_str = std::fs::read_to_string(&fmt_filepath).unwrap();

        assert_eq!(
            post_fmt_str,
            r#"""
        def fn( ):
            pass"#
        );
    }

    #[test]
    fn test_init_project() {
        let dir = tempdir().unwrap().into_path();
        let mut ws = Workspace::from_path(dir).unwrap();

        init_project(&mut ws).unwrap();

        let toml_path = ws.project().root().join("pyproject.toml");
        let ser_toml = PyProjectTomlWrapper::from_path(toml_path).unwrap();

        assert_eq!(
            ser_toml.to_string(),
            crate::default_pyproject_toml_contents()
        );
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
    fn test_fix_project_lints() {
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
    fn test_remove_project_dependencies() {
        todo!()
    }

    #[test]
    fn test_remove_project_optional_dependencies() {
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
