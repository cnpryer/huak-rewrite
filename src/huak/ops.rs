///! This module implements various operations to interact with valid workspaces
///! existing on a system.
use crate::{
    error::{HuakError, HuakResult},
    DependencyResolver, Package, Workspace,
};
use std::path::Path;

/// Activate a Python virtual environment.
pub fn activate_venv(workspace: &Workspace) -> HuakResult<()> {
    let env = match workspace.python_environments.get("default") {
        Some(it) => it,
        None => return Err(HuakError::PyVenvNotFoundError),
    };

    env.activate_with(workspace.platform.terminal())
}

/// Add Python packages as a dependencies to a Python project.
pub fn add_project_dependencies(
    workspace: &mut Workspace,
    dependencies: &[Package],
) -> HuakResult<()> {
    let resolver = DependencyResolver::new()
        .with_dependencies(dependencies)?
        .resolve();

    todo!()
}

/// Add Python packages as optional dependencies to a Python project.
pub fn add_project_optional_dependencies(
    workspace: &Workspace,
    dependencies: &[Package],
    groups: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Build the Python project as installable package.
pub fn build_project(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// Clean the dist directory.
pub fn clean_project(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// Format the Python project's source code.
pub fn format_project(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// Initilize an existing Python project.
pub fn init_project(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// Install a Python project's dependencies to an environment.
pub fn install_project_dependencies(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// Install groups of a Python project's optional dependencies to an environment.
pub fn install_project_optional_dependencies(
    workspace: &Workspace,
    groups: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Lint a Python project's source code.
pub fn lint_project(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// Fix any fixable lint issues found in the Python project's source code.
pub fn fix_project_lints(workspace: &Workspace) -> HuakResult<()> {
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
pub fn remove_project_dependencies(
    workspace: &Workspace,
    dependency_names: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Remove a dependency from a Python project.
pub fn remove_project_optional_dependencies(
    workspace: &Workspace,
    dependency_names: &[&str],
    groups: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_str_with_context(workspace: &Workspace, command: &str) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_list_with_context(workspace: &Workspace, command: &[&str]) -> HuakResult<()> {
    todo!()
}

/// Run a Python project's tests.
pub fn test_project(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// Display the version of the Python project.
pub fn display_project_version(workspace: &Workspace) -> HuakResult<()> {
    todo!()
}

/// NOTE: Operations are meant to be executed on projects and environments.
///       See https://github.com/cnpryer/huak/issues/123
///       To run some of these tests a .venv must be available at the project's root.
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tempfile::tempdir;

    use crate::{test_resources_dir_path, PyProjectToml, VirtualEnvironment};

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
        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        add_project_dependencies(&ws, &deps).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let project = ws.project();
        let python_environment = ws.python_environments.get("default").unwrap();
        let ser_toml =
            PyProjectToml::from_path(dir.join("mock-project").join("pyproject.toml")).unwrap();

        assert!(python_environment
            .find_site_packages_package("ruff")
            .is_some());
        assert!(deps
            .iter()
            .all(|item| project.dependencies().contains(item)));
        assert!(deps
            .iter()
            .map(|item| item)
            .all(|item| ser_toml.dependencies().contains(&item.dependency_str())));
        assert!(deps.iter().map(|item| item).all(|item| project
            .pyproject_toml()
            .dependencies()
            .contains(&item.dependency_str())));
    }

    #[test]
    fn test_add_optional_project_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let deps = [Package::from_str("ruff").unwrap()];
        let groups = ["test"];
        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        add_project_optional_dependencies(&ws, &deps, &groups).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let project = ws.project();
        let python_environment = ws.python_environments.get("default").unwrap();
        let ser_toml =
            PyProjectToml::from_path(dir.join("mock-project").join("pyproject.toml")).unwrap();

        assert!(python_environment
            .find_site_packages_package("ruff")
            .is_some());
        assert!(deps
            .iter()
            .all(|item| project.optional_dependencey_group("test").contains(item)));
        assert!(deps.iter().map(|item| item).all(|item| ser_toml
            .optional_dependencey_group("test")
            .contains(&item.dependency_str())));
        assert!(deps.iter().map(|item| item).all(|item| project
            .pyproject_toml()
            .optional_dependencey_group("test")
            .contains(&item.dependency_str())));
    }

    #[test]
    fn test_build_project() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let python_environment = VirtualEnvironment::from_path(".venv").unwrap();
        ws.add_python_environment("default", python_environment)
            .unwrap();

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
        let ser_toml = PyProjectToml::from_path(toml_path).unwrap();

        assert_eq!(
            ser_toml.to_string_pretty().unwrap(),
            crate::default_pyproject_toml_contents()
        );
    }

    #[test]
    fn test_install_project_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let python_environment = ws.python_environments.get_mut("default").unwrap();
        python_environment.uninstall_package("black").unwrap();
        let had_black = python_environment
            .find_site_packages_package("black")
            .is_some();

        install_project_dependencies(&ws).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();

        assert!(!had_black);
        assert!(ws
            .python_environments
            .get("default")
            .unwrap()
            .find_site_packages_package("black")
            .is_some());
    }

    #[test]
    fn test_install_project_optional_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let env = ws.python_environments.get_mut("default").unwrap();
        env.uninstall_package("pytest").unwrap();
        let had_pytest = env.find_site_packages_package("pytest").is_some();

        install_project_optional_dependencies(&ws, &["test"]).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();

        assert!(!had_pytest);
        assert!(ws
            .python_environments
            .get("default")
            .unwrap()
            .find_site_packages_package("pytest")
            .is_some());
    }

    #[test]
    fn test_lint_project() {
        todo!()
    }

    #[test]
    fn test_fix_project_lints() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let lint_fix_filepath = ws
            .project()
            .root()
            .join("src")
            .join("mock_project")
            .join("fix_me.py");
        let pre_fix_str = r#"
import json # this gets removed(autofixed)


def fn():
    pass
"#;
        let expected = r#"


def fn():
    pass
"#;
        std::fs::write(&lint_fix_filepath, pre_fix_str).unwrap();

        fix_project_lints(&ws).unwrap();

        let post_fix_str = std::fs::read_to_string(&lint_fix_filepath).unwrap();

        assert_eq!(post_fix_str, expected);
    }

    #[test]
    fn test_new_default_project() {
        let dir = tempdir().unwrap().into_path();
        let had_toml = dir.join("mock-project").join("pyproject.toml").exists();

        create_new_default_project(dir.join("mock-project")).unwrap();

        assert!(!had_toml);
        assert!(dir.join("mock-project").join("pyproject.toml").exists());
    }

    #[test]
    fn test_new_lib_project() {
        let dir = tempdir().unwrap().into_path();

        create_new_lib_project(dir.join("mock-project")).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let test_file_filepath = ws.project().root().join("tests").join("test_version.py");
        let test_file = std::fs::read_to_string(&test_file_filepath).unwrap();
        let expected_test_file = format!(
            r#"from {} import __version__


def test_version():
    __version__
"#,
            ws.project().pyproject_toml().project_name()
        );
        let init_file_filepath = ws
            .project()
            .root()
            .join("src")
            .join("project")
            .join("__init__.py");
        let init_file = std::fs::read_to_string(&init_file_filepath).unwrap();
        let expected_init_file = "__version__ = \"0.0.1\"
";

        assert!(ws
            .project()
            .pyproject_toml()
            .inner
            .project
            .as_ref()
            .unwrap()
            .scripts
            .is_none());
        assert_eq!(test_file, expected_test_file);
        assert_eq!(init_file, expected_init_file);
    }

    #[test]
    fn test_new_app_project() {
        let dir = tempdir().unwrap().into_path();

        create_new_app_project(dir.join("mock-project")).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let ser_toml = ws.project().pyproject_toml();
        let main_file_filepath = ws
            .project()
            .root()
            .join("src")
            .join("project")
            .join("main.py");
        let main_file = std::fs::read_to_string(&main_file_filepath).unwrap();
        let expected_main_file = "\
def main():
print(\"Hello, World!\")


if __name__ == \"__main__\":
main()
";

        assert_eq!(
            ser_toml
                .inner
                .project
                .as_ref()
                .unwrap()
                .scripts
                .as_ref()
                .unwrap()[ser_toml.project_name()],
            format!("{}.main:main", ser_toml.project_name())
        );
        assert_eq!(main_file, expected_main_file);

        assert!(ser_toml.inner.project.as_ref().unwrap().scripts.is_some());
    }

    #[ignore = "currently untestable"]
    #[test]
    fn test_publish_project() {
        todo!()
    }

    #[test]
    fn test_remove_project_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let black_package = ws
            .python_environments
            .get("default")
            .unwrap()
            .find_site_packages_package("black");
        let env_had_black = black_package.is_some();
        let black_package = black_package.unwrap();
        let toml_had_black = ws
            .project()
            .pyproject_toml()
            .dependencies()
            .contains(&black_package.dependency_str());

        remove_project_dependencies(&ws, &[black_package.name()]).unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let env_has_black = ws
            .python_environments
            .get_mut("default")
            .unwrap()
            .find_site_packages_package("black")
            .is_some();
        let toml_has_black = ws
            .project()
            .pyproject_toml()
            .dependencies()
            .contains(&black_package.dependency_str());
        ws.python_environments
            .get_mut("default")
            .unwrap()
            .install_package(&black_package)
            .unwrap();

        assert!(env_had_black);
        assert!(toml_had_black);
        assert!(!env_has_black);
        assert!(!toml_has_black);
    }

    #[test]
    fn test_remove_project_optional_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let pytest_package = ws
            .python_environments
            .get("default")
            .unwrap()
            .find_site_packages_package("pytest");
        let env_had_pytest = pytest_package.is_some();
        let pytest_package = pytest_package.unwrap();
        let toml_had_pytest = ws
            .project()
            .pyproject_toml()
            .dependencies()
            .contains(&pytest_package.dependency_str());

        remove_project_optional_dependencies(&ws, &[pytest_package.name()], &["test"]).unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let env_has_pytest = ws
            .python_environments
            .get_mut("default")
            .unwrap()
            .find_site_packages_package("pytest")
            .is_some();
        let toml_has_pytest = ws
            .project()
            .pyproject_toml()
            .dependencies()
            .contains(&pytest_package.dependency_str());
        ws.python_environments
            .get_mut("default")
            .unwrap()
            .install_package(&pytest_package)
            .unwrap();

        assert!(env_had_pytest);
        assert!(toml_had_pytest);
        assert!(!env_has_pytest);
        assert!(!toml_has_pytest);
    }

    #[test]
    fn test_run_command_with_context() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let env_had_xlcsv = ws
            .python_environments
            .get_mut("default")
            .unwrap()
            .find_site_packages_package("xlcsv")
            .is_some();

        run_command_str_with_context(&ws, "pip install xlcsv").unwrap();

        let mut ws = Workspace::from_path(dir.join("mock-project")).unwrap();
        let env = ws.python_environments.get_mut("default").unwrap();
        let env_has_xlcsv = env.find_site_packages_package("xlcsv").is_some();
        env.uninstall_package("xlcsv").unwrap();

        assert!(!env_had_xlcsv);
        assert!(env_has_xlcsv);
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
