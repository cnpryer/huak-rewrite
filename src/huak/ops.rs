///! This module implements various operations to interact with valid workspaces
///! existing on a system.
use crate::{
    error::{HuakError, HuakResult},
    sys::{self, Terminal},
    Package, Project, PyProjectToml, VirtualEnvironment,
};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub struct OperationConfig {
    root: PathBuf,
    build_options: Option<BuildOptions>,
    format_options: Option<FormatOptions>,
    lint_options: Option<LintOptions>,
}

impl OperationConfig {
    pub fn new() -> OperationConfig {
        OperationConfig {
            root: PathBuf::new(),
            build_options: None,
            format_options: None,
            lint_options: None,
        }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn with_root(&mut self, path: impl AsRef<Path>) -> &mut OperationConfig {
        self.root = path.as_ref().to_path_buf();
        self
    }

    pub fn build_options(&self) -> Option<&BuildOptions> {
        self.build_options.as_ref()
    }

    pub fn with_build_options(&mut self, options: BuildOptions) -> &mut OperationConfig {
        self.build_options = Some(options);
        self
    }

    pub fn format_options(&self) -> Option<&FormatOptions> {
        self.format_options.as_ref()
    }

    pub fn with_format_options(&mut self, options: FormatOptions) -> &mut OperationConfig {
        self.format_options = Some(options);
        self
    }

    pub fn lint_options(&self) -> Option<&LintOptions> {
        self.lint_options.as_ref()
    }

    pub fn with_lint_options(&mut self, options: LintOptions) -> &mut OperationConfig {
        self.lint_options = Some(options);
        self
    }
}

pub struct BuildOptions;

impl<'a> Iterator for &'a BuildOptions {
    type Item = (String, String); // TODO

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct FormatOptions;

impl<'a> Iterator for &'a FormatOptions {
    type Item = (String, String); // TODO

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct LintOptions;

impl<'a> Iterator for &'a LintOptions {
    type Item = (String, String); // TODO

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Activate a Python virtual environment.
pub fn activate_venv(config: &OperationConfig) -> HuakResult<()> {
    let venv = VirtualEnvironment::find(Some(config.root()))?;
    let terminal = Terminal::new();
    venv.activate_with(&terminal)
}

/// Add Python packages as a dependencies to a Python project.
pub fn add_project_dependencies(
    config: &OperationConfig,
    dependencies: &[Package],
) -> HuakResult<()> {
    let mut venv = VirtualEnvironment::find(Some(config.root()))?;
    let mut packages = venv.installed_packages()?;
    packages.extend_from_slice(dependencies);
    venv.install_packages(&packages)?;
    let manifest_path = config.root().join("pyproject.toml");
    let mut project = Project::from_manifest(&manifest_path)?;
    for package in &packages {
        project.add_dependency(package);
    }
    project.pyproject_toml().write_file(&manifest_path)
}

/// Add Python packages as optional dependencies to a Python project.
pub fn add_project_optional_dependencies(
    config: &OperationConfig,
    dependencies: &[Package],
    group: &str,
) -> HuakResult<()> {
    let mut venv = VirtualEnvironment::find(Some(config.root()))?;
    let mut packages = venv.installed_packages()?;
    packages.extend_from_slice(dependencies);
    venv.install_packages(&packages)?;
    let manifest_path = config.root().join("pyproject.toml");
    let mut project = Project::from_manifest(&manifest_path)?;
    for package in &packages {
        project.add_optional_dependency(package, group);
    }
    project.pyproject_toml().write_file(&manifest_path)
}

/// Build the Python project as installable package.
pub fn build_project(config: &OperationConfig) -> HuakResult<()> {
    let venv = VirtualEnvironment::find(Some(config.root()))?;
    let mut terminal = Terminal::new();
    let mut paths = sys::env_path_values();
    paths.insert(0, venv.root().to_path_buf());
    let mut cmd = Command::new("build");
    let mut cmd = cmd
        .env(
            "PATH",
            std::env::join_paths(paths).map_err(|e| HuakError::InternalError(e.to_string()))?,
        )
        .current_dir(config.root());
    let build_options = match config.build_options() {
        Some(it) => it,
        None => return Err(HuakError::BuildOptionsMissingError),
    };
    for vals in build_options {
        cmd = cmd.arg(format!("--{}={}", vals.0, vals.1));
    }
    terminal.run_command(cmd)
}

/// Clean the dist directory.
pub fn clean_project(config: &OperationConfig) -> HuakResult<()> {
    todo!()
}

/// Format the Python project's source code.
pub fn format_project(config: &OperationConfig) -> HuakResult<()> {
    let venv = VirtualEnvironment::find(Some(config.root()))?;
    let mut terminal = Terminal::new();
    let mut paths = sys::env_path_values();
    paths.insert(0, venv.root().to_path_buf());
    let mut cmd = Command::new("black");
    let mut cmd = cmd
        .env(
            "PATH",
            std::env::join_paths(paths).map_err(|e| HuakError::InternalError(e.to_string()))?,
        )
        .current_dir(config.root())
        .arg(".");
    let format_options = match config.format_options() {
        Some(it) => it,
        None => return Err(HuakError::BuildOptionsMissingError),
    };
    for vals in format_options {
        cmd = cmd.arg(format!("--{}={}", vals.0, vals.1));
    }
    terminal.run_command(cmd)
}

/// Initilize an existing Python project.
pub fn init_project(config: &OperationConfig) -> HuakResult<()> {
    let manifest_path = config.root().join("pyproject.toml");
    let pyproject_toml = PyProjectToml::default();
    pyproject_toml.write_file(manifest_path)
}

/// Install a Python project's dependencies to an environment.
pub fn install_project_dependencies(config: &OperationConfig) -> HuakResult<()> {
    let mut venv = VirtualEnvironment::find(Some(config.root()))?;
    let project = Project::from_manifest(config.root().join("pyproject.toml"))?;
    let packages = project.dependencies();
    venv.install_packages(packages)
}

/// Install groups of a Python project's optional dependencies to an environment.
pub fn install_project_optional_dependencies(
    config: &OperationConfig,
    group: &str,
) -> HuakResult<()> {
    let mut venv = VirtualEnvironment::find(Some(config.root()))?;
    let project = Project::from_manifest(config.root().join("pyproject.toml"))?;
    let packages = project.optional_dependencey_group(group);
    venv.install_packages(packages)
}

/// Lint a Python project's source code.
pub fn lint_project(config: &OperationConfig) -> HuakResult<()> {
    let venv = VirtualEnvironment::find(Some(config.root()))?;
    let mut terminal = Terminal::new();
    let mut paths = sys::env_path_values();
    paths.insert(0, venv.root().to_path_buf());
    let mut cmd = Command::new("ruff");
    let mut cmd = cmd
        .env(
            "PATH",
            std::env::join_paths(paths).map_err(|e| HuakError::InternalError(e.to_string()))?,
        )
        .current_dir(config.root())
        .arg(".");
    let lint_options = match config.lint_options() {
        Some(it) => it,
        None => return Err(HuakError::BuildOptionsMissingError),
    };
    for vals in lint_options {
        cmd = cmd.arg(format!("--{}={}", vals.0, vals.1));
    }
    terminal.run_command(cmd)
}

/// Fix any fixable lint issues found in the Python project's source code.
pub fn fix_project_lints(config: &OperationConfig) -> HuakResult<()> {
    let venv = VirtualEnvironment::find(Some(config.root()))?;
    let mut terminal = Terminal::new();
    let mut paths = sys::env_path_values();
    paths.insert(0, venv.root().to_path_buf());
    let mut cmd = Command::new("ruff");
    let mut cmd = cmd
        .env(
            "PATH",
            std::env::join_paths(paths).map_err(|e| HuakError::InternalError(e.to_string()))?,
        )
        .current_dir(config.root())
        .arg(".")
        .arg("--fix");
    let lint_options = match config.lint_options() {
        Some(it) => it,
        None => return Err(HuakError::BuildOptionsMissingError),
    };
    for vals in lint_options {
        cmd = cmd.arg(format!("--{}={}", vals.0, vals.1));
    }
    terminal.run_command(cmd)
}

/// Create a new Python project with all defaults on the system.
pub fn create_new_default_project(config: &OperationConfig) -> HuakResult<()> {
    let project = Project::default();
    project.write_project(config.root())
}

/// Create a new library-like Python project on the system.
pub fn create_new_lib_project(config: &OperationConfig) -> HuakResult<()> {
    let project = Project::new_lib();
    project.write_project(config.root())
}

/// Create a new application-like Python project on the system.
pub fn create_new_app_project(config: &OperationConfig) -> HuakResult<()> {
    let project = Project::new_lib();
    project.write_project(config.root())
}

/// Remove a dependency from a Python project.
pub fn remove_project_dependencies(
    config: &OperationConfig,
    dependency_names: &[&str],
) -> HuakResult<()> {
    todo!()
}

/// Remove a dependency from a Python project.
pub fn remove_project_optional_dependencies(
    config: &OperationConfig,
    dependency_names: &[&str],
    group: &str,
) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_str_with_context(config: &OperationConfig, command: &str) -> HuakResult<()> {
    todo!()
}

/// Run a command from within a Python project's context.
pub fn run_command_list_with_context(config: &OperationConfig, command: &[&str]) -> HuakResult<()> {
    todo!()
}

/// Run a Python project's tests.
pub fn test_project(config: &OperationConfig) -> HuakResult<()> {
    todo!()
}

/// Display the version of the Python project.
pub fn display_project_version(config: &OperationConfig) -> HuakResult<()> {
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
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));

        add_project_dependencies(&config, &deps).unwrap();

        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let ser_toml =
            PyProjectToml::from_path(dir.join("mock-project").join("pyproject.toml")).unwrap();

        assert!(venv.find_site_packages_package("ruff").is_some());
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
        let group = "test";
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));

        add_project_optional_dependencies(&config, &deps, group).unwrap();

        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let ser_toml =
            PyProjectToml::from_path(dir.join("mock-project").join("pyproject.toml")).unwrap();

        assert!(venv.find_site_packages_package("ruff").is_some());
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
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let venv = VirtualEnvironment::from_path(".venv").unwrap();

        build_project(&config).unwrap();
    }

    #[test]
    fn test_clean_project() {
        todo!()
    }

    #[test]
    fn test_fmt_project() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let fmt_filepath = project
            .root()
            .join("src")
            .join("mock_project")
            .join("fmt_me.py");
        let pre_fmt_str = r#"""
def fn( ):
    pass"#;
        std::fs::write(&fmt_filepath, pre_fmt_str).unwrap();

        format_project(&config).unwrap();

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
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));

        init_project(&config).unwrap();

        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let toml_path = project.root().join("pyproject.toml");
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
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let mut venv = VirtualEnvironment::from_path(".venv").unwrap();
        venv.uninstall_package("black").unwrap();
        let had_black = venv.find_site_packages_package("black").is_some();

        install_project_dependencies(&config).unwrap();

        assert!(!had_black);
        assert!(venv.find_site_packages_package("black").is_some());
    }

    #[test]
    fn test_install_project_optional_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let mut venv = VirtualEnvironment::from_path(".venv").unwrap();
        venv.uninstall_package("pytest").unwrap();
        let had_pytest = venv.find_site_packages_package("pytest").is_some();

        install_project_optional_dependencies(&config, "test").unwrap();

        assert!(!had_pytest);
        assert!(venv.find_site_packages_package("pytest").is_some());
    }

    #[test]
    fn test_lint_project() {
        todo!()
    }

    #[test]
    fn test_fix_project_lints() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let lint_fix_filepath = project
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

        fix_project_lints(&config).unwrap();

        let post_fix_str = std::fs::read_to_string(&lint_fix_filepath).unwrap();

        assert_eq!(post_fix_str, expected);
    }

    #[test]
    fn test_new_default_project() {
        let dir = tempdir().unwrap().into_path();
        let had_toml = dir.join("mock-project").join("pyproject.toml").exists();
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));

        create_new_default_project(&config).unwrap();

        assert!(!had_toml);
        assert!(dir.join("mock-project").join("pyproject.toml").exists());
    }

    #[test]
    fn test_new_lib_project() {
        let dir = tempdir().unwrap().into_path();
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));

        create_new_lib_project(&config).unwrap();

        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let test_file_filepath = project.root().join("tests").join("test_version.py");
        let test_file = std::fs::read_to_string(&test_file_filepath).unwrap();
        let expected_test_file = format!(
            r#"from {} import __version__


def test_version():
    __version__
"#,
            project.pyproject_toml().project_name()
        );
        let init_file_filepath = project
            .root()
            .join("src")
            .join("project")
            .join("__init__.py");
        let init_file = std::fs::read_to_string(&init_file_filepath).unwrap();
        let expected_init_file = "__version__ = \"0.0.1\"
";

        assert!(project
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
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));

        create_new_app_project(&config).unwrap();

        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let ser_toml = project.pyproject_toml();
        let main_file_filepath = project.root().join("src").join("project").join("main.py");
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
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let black_package = venv.find_site_packages_package("black");
        let venv_had_black = black_package.is_some();
        let black_package = black_package.unwrap();
        let toml_had_black = project
            .pyproject_toml()
            .dependencies()
            .contains(&black_package.dependency_str());

        remove_project_dependencies(&config, &[black_package.name()]).unwrap();

        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let mut venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let venv_has_black = venv.find_site_packages_package("black").is_some();
        let toml_has_black = project
            .pyproject_toml()
            .dependencies()
            .contains(&black_package.dependency_str());
        venv.install_package(&black_package).unwrap();

        assert!(venv_had_black);
        assert!(toml_had_black);
        assert!(!venv_has_black);
        assert!(!toml_has_black);
    }

    #[test]
    fn test_remove_project_optional_dependencies() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let pytest_package = venv.find_site_packages_package("pytest");
        let venv_had_pytest = pytest_package.is_some();
        let pytest_package = pytest_package.unwrap();
        let toml_had_pytest = project
            .pyproject_toml()
            .dependencies()
            .contains(&pytest_package.dependency_str());

        remove_project_optional_dependencies(&config, &[pytest_package.name()], "test").unwrap();

        let project = Project::from_manifest(config.root().join("pyproject.toml")).unwrap();
        let mut venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let venv_has_pytest = venv.find_site_packages_package("pytest").is_some();
        let toml_has_pytest = project
            .pyproject_toml()
            .dependencies()
            .contains(&pytest_package.dependency_str());
        venv.install_package(&pytest_package).unwrap();

        assert!(venv_had_pytest);
        assert!(toml_had_pytest);
        assert!(!venv_has_pytest);
        assert!(!toml_has_pytest);
    }

    #[test]
    fn test_run_command_with_context() {
        let dir = tempdir().unwrap().into_path();
        crate::fs::copy_dir(&test_resources_dir_path().join("mock-project"), &dir).unwrap();
        let mut config = OperationConfig::new();
        let config = config.with_root(dir.join("mock-project"));
        let venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let venv_had_xlcsv = venv.find_site_packages_package("xlcsv").is_some();

        run_command_str_with_context(&config, "pip install xlcsv").unwrap();

        let mut venv = VirtualEnvironment::from_path(PathBuf::from(".venv")).unwrap();
        let venv_has_xlcsv = venv.find_site_packages_package("xlcsv").is_some();
        venv.uninstall_package("xlcsv").unwrap();

        assert!(!venv_had_xlcsv);
        assert!(venv_has_xlcsv);
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
