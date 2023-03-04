use error::HuakResult;
use pep440_rs::Version;
use pyproject_toml::PyProjectToml as InnerPyProjectToml;
use std::{collections::HashMap, fs::File, path::PathBuf};

mod error;
mod fs;

const DEFAULT_VENV_NAME: &str = ".venv";

#[cfg(test)]
/// The resource directory found in the Huak repo used for testing purposes.
pub(crate) fn test_resources_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources")
}

/// A Python project can be anything from a script to automate some process to a
/// production web application. Projects consist of Python source code and a
/// project-marking `pyproject.toml` file. PEPs provide Python’s ecosystem with
/// standardization and Huak leverages them to do many things such as identify your
/// project. See PEP 621.
pub struct Project {
    /// A value to indicate the type of the project (app, library, etc.).
    project_type: ProjectType,
    /// Data about the project's layout.
    project_layout: ProjectLayout,
    /// The project's pyproject.toml file containing metadata about the project.
    /// See https://peps.python.org/pep-0621/
    pyproject_toml: PyProjectToml,
    /// The project's main dependencies.
    dependencies: Vec<Package>,
    /// The project's optional dependencies.
    optional_dependencies: HashMap<String, Vec<Package>>,
}

impl Project {
    /// Create a new `Project`.
    pub fn new() -> Project {
        todo!()
    }

    /// Get the Python project's pyproject.toml file.
    pub fn pyproject_toml(&self) -> &InnerPyProjectToml {
        todo!()
    }

    /// Get the Python project's pyproject.toml data.
    pub fn pyproject_toml_data(&self) -> &PyProjectToml {
        todo!()
    }

    /// Get the Python project's main dependencies.
    fn dependencies(&self) -> &Vec<Package> {
        todo!()
    }

    /// Get a group of optional dependencies from the Python project.
    fn optional_dependencey_group(&self, group: &str) -> &Vec<Package> {
        todo!()
    }

    /// Add a Python package as a dependency to the project.
    fn add_dependency(&mut self, package: Package) {
        todo!()
    }

    /// Add a Python package as a dependency to the project.
    fn add_optional_dependency(&mut self, group: &str, package: Package) {
        todo!()
    }

    /// Remove a dependency from the project.
    fn remove_dependency(&mut self, package_name: &str) {
        todo!()
    }

    /// Remove an optional dependency from the project.
    fn remove_optional_dependency(&self, group: &str, package_name: &str) {
        todo!()
    }
}

/// A project type might indicate if a project is an application-like project or a
/// library-like project.
#[derive(Default, Eq, PartialEq, Debug)]
pub enum ProjectType {
    /// Library-like projects are essentially anything that isn’t an application. An
    /// example would be a typical Python package distributed to PyPI.
    #[default]
    Library,
    /// Application-like projects are projects intended to be distributed as an executed
    /// process. Examples would include web applications, automated scripts, etc..
    Application,
}

/// Data about the project's layout. The project layout includes the location of
/// important files and directories.
pub struct ProjectLayout {
    /// The absolute path to the root directory of the project.
    root: PathBuf,
    /// The absolute path to the Python root directory.
    python_root: PathBuf,
    /// The absolute path to the pyproject.toml file.
    pyproject_toml_path: PathBuf,
}

/// The pyproject.toml struct containing an inner PyProjectToml.
pub struct PyProjectToml {
    /// Inner pyproject.toml
    inner: InnerPyProjectToml,
}

/// Environments can be anything from a model of the system environment to a specific
/// type of Python environment such as a virtual environment.
pub struct Environment {
    /// The environment's configuration data.
    python_config: PythonEnvironmentConfig,
    /// The absolute path to the environment's Python interpreter.
    python_path: PathBuf,
    /// The absolute path to the environment's executables directory.
    python_exeutables_dir_path: PathBuf,
    /// The absolute path to the environment's site-packages directory.
    site_packages_dir_path: PathBuf,
    /// The absolute path to the system executables directory.
    base_exeutables_dir_path: PathBuf,
    /// The absolute path to the system site-packages directory.
    base_site_packages_dir_path: PathBuf,
}

impl Environment {
    /// Create a new `Environement`.
    pub fn new() -> Environment {
        todo!()
    }

    /// Create a Python environment on the system.
    pub fn create() -> HuakResult<Environment> {
        todo!()
    }

    /// The absolute path to the Python environment's python interpreter binary.
    fn python_path(&self) -> &PathBuf {
        todo!()
    }

    /// The version of the Python environment's Python interpreter.
    fn python_version(&self) -> &Version {
        todo!()
    }

    /// The absolute path to the Python interpreter used to create the Python
    /// environment.
    fn base_python_path(&self) -> &PathBuf {
        todo!()
    }

    /// The version of the Python interpreter used to create the Python environment.
    fn base_python_version(&self) -> &Version {
        todo!()
    }

    /// The absolute path to the Python environment's executables directory.
    fn python_executables_dir_path(&self) -> &PathBuf {
        todo!()
    }

    /// The absolute path to the Python environment's site-packages directory.
    fn site_packages_dir_path(&self) -> &PathBuf {
        todo!()
    }

    /// Add a package to the site-packages directory.
    fn add_package_to_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Remove a package from the site-packages directory.
    fn remove_package_from_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Get a package from the site-packages directory if it is already installed.
    fn find_site_packages_package(&self, name: &str) -> Package {
        todo!()
    }

    /// Get a package's dist info from the site-packages directory if it is there.
    fn find_site_packages_dist_info(&self, name: &str) -> DistInfo {
        todo!()
    }

    /// Check if the Python environment is isolated from any system site-packages
    /// directory.
    fn is_isolated(&self) -> &bool {
        todo!()
    }

    /// Activate the Python environment with the system shell.
    fn activate(&self) {
        todo!()
    }

    /// Run a list of values as a command with the Python environment as its context.
    fn run_command_list(&self, list: &[&str]) {
        todo!()
    }

    /// Run a string as a command with the Python environment as its context.
    fn run_command_str(&self, string: &str) {
        todo!()
    }

    /// The absolute path to the system's executables directory.
    fn base_executables_dir_path(&self) -> &PathBuf {
        todo!()
    }

    /// The absolute path to the system's site-packages directory.
    fn base_site_packages_dir(&self) -> &PathBuf {
        todo!()
    }

    /// Add a package to the system's site-packages directory.
    fn add_package_to_base_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Remove a package from the system's site-packages directory.
    fn remove_package_from_base_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Get a package from the system's site-packages directory if it is already
    /// installed.
    fn find_base_site_packages_package(&self, name: &str) -> Package {
        todo!()
    }

    /// Get a package's dist info from the system's site-packages directory if it is
    /// there.
    fn find_base_site_packages_dist_info(&self, name: &str) -> DistInfo {
        todo!()
    }
}

/// Data about some environment's Python configuration. This abstraction is modeled after
/// the pyenv.cfg file used for Python virtual environments.
pub struct PythonEnvironmentConfig {
    /// Path to directory containing the Python installation used to create the
    /// environment.
    home: PathBuf,
    /// Boolean value to indicate if the Python environment is isolated from the global
    /// site.
    include_system_site_packages: bool,
    // The version of the environment's Python interpreter.
    version: Version,
}

/// The python package compliant with packaging.python.og.
pub struct Package {
    /// Name designated to the package by the author(s).
    name: String,
    /// Normalized name of the Python package.
    canonical_name: String,
    /// The package's core metadata.
    /// https://packaging.python.org/en/latest/specifications/core-metadata/
    core_metadata: PackageMetadata,
    /// The PEP 440 version of the package.
    version: Version,
    /// Tags used to indicate platform compatibility.
    platform_tags: Vec<PlatformTag>,
    /// The package's distribtion info.
    distribution_info: Option<DistInfo>,
}

/// Core package metadata.
/// https://packaging.python.org/en/latest/specifications/core-metadata/
pub struct PackageMetadata;

/// Tags used to indicate platform compatibility.
/// https://packaging.python.org/en/latest/specifications/platform-compatibility-tags/
pub enum PlatformTag {}

/// Package distribtion info stored in the site-packages directory adjacent to the
/// installed package artifact.
/// https://peps.python.org/pep-0376/#one-dist-info-directory-per-installed-distribution
pub struct DistInfo {
    /// File containing the name of the tool used to install the package.
    installer_file: File,
    /// File containing the package's license information.
    license_file: Option<File>,
    /// File containing metadata about the package.
    /// See
    ///   https://peps.python.org/pep-0345/
    ///   https://peps.python.org/pep-0314/
    ///   https://peps.python.org/pep-0241/
    metadata_file: File,
    /// File containing each file isntalled as part of the package's installation.
    /// See https://peps.python.org/pep-0376/#record
    record_file: File,
    /// File added to the .dist-info directory of the installed distribution if the
    /// package was explicitly requested.
    /// See https://peps.python.org/pep-0376/#requested
    requested_file: Option<File>,
    /// File containing metadata about the archive.
    wheel_file: Option<File>,
}

/// A simple API to interact with a platform with Python installed.
pub trait PythonPlatform {
    /// Get the absolute path to the Python interpreter installed on the platform.
    fn python_path(&self) -> &PathBuf;
}

/// Data about the some platform.
pub struct Platform {
    /// The name of the platform.
    name: String,
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn project_from_dir() {
        let dir = tempdir().unwrap().into_path();
        let mock_dir = test_resources_dir_path().join("mock-project");
        fs::copy_dir(&mock_dir, &dir).unwrap();

        let project_from_root = Project::from_dir(dir.join("mock-project")).unwrap();
        let project_from_package_root =
            Project::from_dir(dir.join("mock-project").join("src").join("mock_project")).unwrap();

        assert!(project_from_root.is_valid());
        assert!(project_from_package_root.is_valid());
        assert_eq!(project_from_root.root, project_from_package_root.root);
    }

    #[test]
    fn project_bootstrap() {
        let default_project = Project::new("my-project");
        let lib = Project::new_lib("my-project");
        let app = Project::new_app("my-project");

        assert!(default_project.is_valid());
        assert_eq!(default_project.project_type, ProjectType::default());

        assert!(lib.is_valid());
        assert_eq!(lib.project_type, ProjectType::Library);

        assert!(app.is_valid());
        assert_eq!(app.project_type, ProjectType::Application);
    }

    #[test]
    fn toml_from_path() {
        let path = test_resources_dir_path()
            .join("mock-project")
            .join("pyproject.toml");
        let toml = PyProjectToml::from_path(path).unwrap();

        assert!(toml.is_valid());
        assert_eq!(toml.project_name, "mock_project");
        assert_eq!(toml.project_version, "0.0.1");
        assert_eq!(toml.project_authors[0].name, "Chris Pryer");
        assert_eq!(toml.project_authors[0].email, "cnpryer@gmail.com");
    }

    #[test]
    fn toml_to_str() {
        let default_toml = PyProjectToml::default();
        let default_toml_str = default_toml.to_str();

        assert!(!default_toml.is_valid());
        assert_eq!(
            default_toml_str,
            r#"[project]
name = ""
version = "0.0.1"
description = ""

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#
        )
    }

    #[test]
    fn toml_dependencies() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let toml = PyProjectToml::from_path(path).unwrap();

        assert_eq!(
            toml.dependencies(),
            vec!["click==8.1.3", "black==22.8.0", "isort==5.12.0"]
        );
    }

    #[test]
    fn toml_optional_dependencies() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let toml = PyProjectToml::from_path(path).unwrap();

        assert_eq!(
            toml.optional_dependencies("test"),
            vec!["pytest>=6", "mock"]
        );
    }

    #[test]
    fn toml_add_dependency() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let mut toml = PyProjectToml::from_path(path).unwrap();

        toml.add_dependency("test");
        assert!(
            toml.to_str(),
            r#"[project]
[project]
name = "mock_project"
version = "0.0.1"
description = ""
dependencies = ["click==8.1.3", "black==22.8.0", "isort==5.12.0", "test"]

[project.optional-dependencies]
test = ["pytest>=6", "mock"]

[[project.authors]]
name = "Chris Pryer"
email = "cnpryer@gmail.com"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#
        )
    }

    #[test]
    fn toml_add_optional_dependency() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let mut toml = PyProjectToml::from_path(path).unwrap();

        toml.add_optional_dependency("test", "test");
        toml.add_optional_dependency("new", "test");
        assert!(
            toml.to_str(),
            r#"[project]
[project]
name = "mock_project"
version = "0.0.1"
description = ""
dependencies = ["click==8.1.3", "black==22.8.0", "isort==5.12.0"]

[project.optional-dependencies]
test = ["pytest>=6", "mock", "test"]
new = ["test"]

[[project.authors]]
name = "Chris Pryer"
email = "cnpryer@gmail.com"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#
        )
    }

    #[test]
    fn toml_remove_dependency() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let mut toml = PyProjectToml::from_path(path).unwrap();

        toml.remove_dependency("isort");
        assert!(
            toml.to_str(),
            r#"[project]
[project]
name = "mock_project"
version = "0.0.1"
description = ""
dependencies = ["click==8.1.3", "black==22.8.0"]

[project.optional-dependencies]
test = ["pytest>=6", "mock"]

[[project.authors]]
name = "Chris Pryer"
email = "cnpryer@gmail.com"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#
        )
    }

    #[test]
    fn toml_remove_optional_dependency() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let mut toml = PyProjectToml::from_path(path).unwrap();

        toml.remove_optional_dependency("mock");
        assert!(
            toml.to_str(),
            r#"[project]
[project]
name = "mock_project"
version = "0.0.1"
description = ""
dependencies = ["click==8.1.3", "black==22.8.0"]

[project.optional-dependencies]
test = ["pytest>=6"]

[[project.authors]]
name = "Chris Pryer"
email = "cnpryer@gmail.com"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#
        )
    }

    #[test]
    fn environment_default() {
        let env = Environment::default();

        assert!(env.is_valid());
        assert_eq!(
            env.python_path.parent().unwrap().parent().unwrap(),
            DEFAULT_VENV_NAME
        );
    }

    #[test]
    fn environment_with_venv() {
        let dir = tempdir().unwrap().into_path();
        let env = Environment::with_venv_path(dir.join(".venv"));
        let venv_root = env.python_path.parent().unwrap().parent().unwrap();

        assert!(env.is_valid());
        assert_eq!(venv_root, dir.join(".venv"));
        assert!(venv_root.join("pyvenv.cfg").exists());
    }

    #[test]
    /// NOTE: This test depends on local virtual environment.
    fn environment_with_find_venv() {
        let mut env = Environment::with_find_venv();
        let venv_root = env.python_path.parent().unwrap().parent().unwrap();

        assert!(env.is_valid());
        assert_eq!(venv_root, dir.join(".venv"));
        assert!(venv_root.join("pyvenv.cfg").exists());
    }

    #[test]
    /// NOTE: This test depends on local virtual environment.
    fn environment_executable_dir_name() {
        let mut env = Environment::with_find_venv();

        assert!(env.python_executables_dir_path.exists())
    }

    #[test]
    /// NOTE: This test depends on local virtual environment.
    fn environment_python_config() {
        let mut env = Environment::with_find_venv();
        let venv_root = env.python_path.parent().unwrap().parent().unwrap();

        assert_eq!(
            env.python_config.to_str(),
            std::fs::read_to_string(venv_root.join("pyvenv.cfg"))
        );
    }

    #[test]
    fn package_display_str() {
        let package = Package::new("package", "==", "0.0.0");

        assert_eq!(package.display(), "package==0.0.0");
    }

    #[test]
    fn package_version_operator() {
        let package = Package::new("package", "==", "0.0.0");

        assert_eq!(package.version_operator, pep440_rs::Operator::Equal);
    }

    #[test]
    fn package_from_str() {
        let package = Package::from_str("package==0.0.0");

        assert_eq!(package.display(), "package==0.0.0");
        assert_eq!(package.name, "package");
        assert_eq!(package.version_operator, pep440_rs::Operator::Equal);
        assert_eq!(package.version_str, "0.0.0");
    }

    #[ignore = "currently untestable"]
    #[test]
    fn package_with_multiple_version_specifiers() {
        todo!()
    }

    #[test]
    fn package_platform_tags() {
        todo!()
    }

    #[test]
    fn package_core_metadata() {
        let package_metdata = Package::new();

        todo!()
    }

    #[test]
    fn package_dist_info() {
        let package_dist_info = Package::new();

        todo!();
    }

    #[test]
    fn platform_shell_command() {
        let platform = Platform::new();

        todo!()
    }

    /// NOTE: This test requires Python to be installed.
    #[test]
    fn platform_python_search() {
        let platform = Platform::with_find_venv();

        assert!(platform.installed_python_path.exists())
    }
}
