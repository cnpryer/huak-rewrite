use error::HuakResult;
use pep440_rs::{Operator as VersionOperator, Version};
use pyproject_toml::PyProjectToml;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

mod error;
mod fs;

const DEFAULT_VENV_NAME: &str = ".venv";
const DEFAULT_PYPROJECT_TOML_STR: &str = r#"[project]
name = ""
version = "0.0.1"
description = ""

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#;

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
#[derive(Default)]
pub struct Project {
    /// A value to indicate the type of the project (app, library, etc.).
    project_type: ProjectType,
    /// Data about the project's layout.
    project_layout: ProjectLayout,
    /// The project's pyproject.toml file containing metadata about the project.
    /// See https://peps.python.org/pep-0621/
    pyproject_toml_wrapper: PyProjectTomlWrapper,
    /// The project's main dependencies.
    dependencies: Vec<Package>,
    /// The project's optional dependencies.
    optional_dependencies: HashMap<String, Vec<Package>>,
}

impl Project {
    /// Create a new project.
    pub fn new() -> Project {
        todo!()
    }

    /// Create a new library-like project.
    pub fn new_lib() -> Project {
        let project_type = ProjectType::Library;

        todo!();
    }

    /// Create a new application-like project.
    pub fn new_app() -> Project {
        let project_type = ProjectType::Application;

        todo!()
    }

    /// Initialize a project from a directory, searching for a pyproject.toml to mark
    /// the project's root directory.
    pub fn from_dir(path: impl AsRef<Path>) -> HuakResult<Project> {
        todo!()
    }

    /// Get the absolute path to the root directory of the project.
    pub fn root(&self) -> &PathBuf {
        todo!()
    }

    /// Get the Python project's pyproject.toml file.
    pub fn pyproject_toml(&self) -> &PyProjectToml {
        todo!()
    }

    /// Get a wrapper around the PyProjectToml data.
    pub fn pyproject_toml_wrapper(&self) -> &PyProjectTomlWrapper {
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

    /// Check if the project is setup correctly.
    fn is_valid(&self) -> bool {
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
#[derive(Default)]
pub struct ProjectLayout {
    /// The absolute path to the root directory of the project.
    root: PathBuf,
    /// The absolute path to the Python root directory.
    python_root: PathBuf,
    /// The absolute path to the pyproject.toml file.
    pyproject_toml_path: PathBuf,
}

/// The pyproject.toml struct containing an inner PyProjectToml.
pub struct PyProjectTomlWrapper {
    /// Inner pyproject.toml
    inner: PyProjectToml,
}

impl Default for PyProjectTomlWrapper {
    fn default() -> Self {
        Self {
            inner: PyProjectToml::new(DEFAULT_PYPROJECT_TOML_STR)
                .expect("could not initilize default pyproject.toml"),
        }
    }
}

impl PyProjectTomlWrapper {
    // Create new pyproject.toml data.
    pub fn new() -> PyProjectTomlWrapper {
        todo!()
    }

    /// Create new pyproject.toml data from a pyproject.toml's path.
    pub fn from_path(path: impl AsRef<Path>) -> HuakResult<PyProjectTomlWrapper> {
        todo!()
    }

    /// Get the project name.
    pub fn project_name(&self) -> &String {
        todo!()
    }

    /// Get the project version.
    pub fn project_version(&self) -> &String {
        todo!()
    }

    /// Get the Python project's main dependencies.
    pub fn dependencies(&self) -> &Vec<&str> {
        todo!()
    }

    /// Get a group of optional dependencies from the Python project.
    pub fn optional_dependencey_group(&self, group: &str) -> &Vec<&str> {
        todo!()
    }

    /// Add a Python package as a dependency to the project.
    pub fn add_dependency(&mut self, package_str: &str) {
        todo!()
    }

    /// Add a Python package as a dependency to the project.
    pub fn add_optional_dependency(&mut self, group: &str, package_str: &str) {
        todo!()
    }

    /// Remove a dependency from the project.
    pub fn remove_dependency(&mut self, package_name: &str) {
        todo!()
    }

    /// Remove an optional dependency from the project.
    pub fn remove_optional_dependency(&self, group: &str, package_name: &str) {
        todo!()
    }

    /// Check if the project is setup correctly.
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}

impl ToString for PyProjectTomlWrapper {
    /// Serialize the current pyproject.toml data to str.
    fn to_string(&self) -> String {
        todo!()
    }
}

/// Environments can be anything from a model of the system environment to a specific
/// type of Python environment such as a virtual environment.
#[derive(Default)]
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

    /// Initilize an environment with an absolute path to a Python virtual environment.
    pub fn with_venv_path(path: impl AsRef<Path>) -> Environment {
        todo!()
    }

    /// Initilize an environment by finding a local Python virtual environment.
    pub fn with_find_venv() -> HuakResult<Environment> {
        todo!()
    }

    /// Create a Python environment on the system.
    pub fn create() -> HuakResult<Environment> {
        todo!()
    }

    /// The absolute path to the Python environment's python interpreter binary.
    pub fn python_path(&self) -> &PathBuf {
        todo!()
    }

    pub fn python_environment_config(&self) -> PythonEnvironmentConfig {
        todo!()
    }

    /// The version of the Python environment's Python interpreter.
    pub fn python_version(&self) -> &Version {
        todo!()
    }

    /// The absolute path to the Python interpreter used to create the Python
    /// environment.
    pub fn base_python_path(&self) -> &PathBuf {
        todo!()
    }

    /// The version of the Python interpreter used to create the Python environment.
    pub fn base_python_version(&self) -> &Version {
        todo!()
    }

    /// The absolute path to the Python environment's executables directory.
    pub fn python_executables_dir_path(&self) -> &PathBuf {
        todo!()
    }

    /// The absolute path to the Python environment's site-packages directory.
    pub fn site_packages_dir_path(&self) -> &PathBuf {
        todo!()
    }

    /// Add a package to the site-packages directory.
    pub fn add_package_to_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Remove a package from the site-packages directory.
    pub fn remove_package_from_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Get a package from the site-packages directory if it is already installed.
    pub fn find_site_packages_package(&self, name: &str) -> Package {
        todo!()
    }

    /// Get a package's dist info from the site-packages directory if it is there.
    pub fn find_site_packages_dist_info(&self, name: &str) -> DistInfo {
        todo!()
    }

    /// Check if the Python environment is isolated from any system site-packages
    /// directory.
    pub fn is_isolated(&self) -> &bool {
        todo!()
    }

    /// Activate the Python environment with the system shell.
    pub fn activate(&self) {
        todo!()
    }

    /// Run a list of values as a command with the Python environment as its context.
    pub fn run_command_list(&self, list: &[&str]) {
        todo!()
    }

    /// Run a string as a command with the Python environment as its context.
    pub fn run_command_str(&self, string: &str) {
        todo!()
    }

    /// The absolute path to the system's executables directory.
    pub fn base_executables_dir_path(&self) -> &PathBuf {
        todo!()
    }

    /// The absolute path to the system's site-packages directory.
    pub fn base_site_packages_dir_path(&self) -> &PathBuf {
        todo!()
    }

    /// Add a package to the system's site-packages directory.
    pub fn add_package_to_base_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Remove a package from the system's site-packages directory.
    pub fn remove_package_from_base_site_packages(&mut self, package: &Package) {
        todo!()
    }

    /// Get a package from the system's site-packages directory if it is already
    /// installed.
    pub fn find_base_site_packages_package(&self, name: &str) -> Package {
        todo!()
    }

    /// Get a package's dist info from the system's site-packages directory if it is
    /// there.
    pub fn find_base_site_packages_dist_info(&self, name: &str) -> DistInfo {
        todo!()
    }

    /// Check if the `Environment` is configured properly.
    pub fn is_valid(&self) -> bool {
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
    version: Option<Version>,
}

impl ToString for PythonEnvironmentConfig {
    /// Convert the `PythonEnvironmentConfig` to str.
    fn to_string(&self) -> String {
        todo!()
    }
}

impl Default for PythonEnvironmentConfig {
    fn default() -> Self {
        Self {
            home: Default::default(),
            include_system_site_packages: Default::default(),
            version: None,
        }
    }
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
    /// The PEP 400 version operator.
    version_operator: VersionOperator,
    /// Tags used to indicate platform compatibility.
    platform_tags: Vec<PlatformTag>,
    /// The package's distribtion info.
    distribution_info: Option<DistInfo>,
}

impl Package {
    /// Create a new Python package.
    pub fn new() -> Package {
        todo!()
    }

    /// Create a new Python package from str parts.
    pub fn from_str_parts(name: &str, operator: &str, version: &str) -> Package {
        todo!()
    }

    /// Get the name of the package.
    pub fn name(&self) -> &String {
        todo!()
    }

    /// Get the pacakge's PEP440 version operator.
    pub fn version_operator(&self) -> &VersionOperator {
        todo!()
    }

    /// Get the package's PEP440 version.
    pub fn version(&self) -> &Version {
        todo!()
    }

    /// Get the package version str.
    pub fn version_str(&self) -> &str {
        todo!()
    }

    /// Get the formatted str of the package to display.
    pub fn display(&self) -> &str {
        todo!()
    }
}

impl FromStr for Package {
    type Err = pep440_rs::Pep440Error;

    /// Create a Python package from str.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Display for Package {
    /// Format the package string for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
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

/// Data about the some platform.
pub struct Platform {
    /// The name of the platform.
    name: String,
    /// The absolute path to the installed Python interpreter.
    python_path: PathBuf,
}

impl Platform {
    /// Create a new platform.
    pub fn new() -> Platform {
        todo!()
    }

    /// Create a new platform and search for an installed Python interpreter.
    pub fn with_find_python() -> Platform {
        todo!()
    }

    /// Get the absolute path to the installed Python interpreter.
    pub fn python_path(&self) -> &PathBuf {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn project_from_dir() {
        let dir = tempdir().unwrap().into_path();
        let mock_dir = test_resources_dir_path().join("mock-project");
        fs::copy_dir(&mock_dir, &dir).unwrap();

        let project_from_root = Project::from_dir(&dir.join("mock-project")).unwrap();
        let project_from_package_root =
            Project::from_dir(&dir.join("mock-project").join("src").join("mock_project")).unwrap();

        assert!(project_from_root.is_valid());
        assert!(project_from_package_root.is_valid());
        assert_eq!(project_from_root.root(), project_from_package_root.root());
    }

    #[test]
    fn project_bootstrap() {
        let default_project = Project::new();
        let lib = Project::new_lib();
        let app = Project::new_app();

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
        let toml = PyProjectTomlWrapper::from_path(&path).unwrap();

        assert!(toml.is_valid());
        assert_eq!(toml.project_name(), "mock_project");
        assert_eq!(toml.project_version(), "0.0.1");
    }

    #[test]
    fn toml_to_str() {
        let default_toml = PyProjectTomlWrapper::default();
        let default_toml_str = default_toml.to_string();

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
        let toml = PyProjectTomlWrapper::from_path(path).unwrap();

        assert_eq!(
            toml.dependencies().deref(),
            vec!["click==8.1.3", "black==22.8.0", "isort==5.12.0"]
        );
    }

    #[test]
    fn toml_optional_dependencies() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let toml = PyProjectTomlWrapper::from_path(path).unwrap();

        assert_eq!(
            toml.optional_dependencey_group("test").deref(),
            vec!["pytest>=6", "mock"]
        );
    }

    #[test]
    fn toml_add_dependency() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let mut toml = PyProjectTomlWrapper::from_path(path).unwrap();

        toml.add_dependency("test");
        assert_eq!(
            toml.to_string(),
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
        let mut toml = PyProjectTomlWrapper::from_path(path).unwrap();

        toml.add_optional_dependency("test", "test");
        toml.add_optional_dependency("new", "test");
        assert_eq!(
            toml.to_string(),
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
        let mut toml = PyProjectTomlWrapper::from_path(path).unwrap();

        toml.remove_dependency("isort");
        assert_eq!(
            toml.to_string(),
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
        let mut toml = PyProjectTomlWrapper::from_path(path).unwrap();

        toml.remove_optional_dependency("test", "mock");
        assert_eq!(
            toml.to_string(),
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
            env.python_path()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .file_name()
                .unwrap(),
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
        let mut env = Environment::with_find_venv().unwrap();
        let venv_root = env.python_path().parent().unwrap().parent().unwrap();

        assert!(env.is_valid());
        assert_eq!(
            venv_root,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".venv")
        );
        assert!(venv_root.join("pyvenv.cfg").exists());
    }

    #[test]
    /// NOTE: This test depends on local virtual environment.
    fn environment_executable_dir_name() {
        let mut env = Environment::with_find_venv().unwrap();

        assert!(env.python_executables_dir_path().exists());
        #[cfg(unix)]
        assert!(env.python_executables_dir_path().join("python").exists());
        #[cfg(windows)]
        assert!(env
            .python_executables_dir_path()
            .join("python.exe")
            .exists());
    }

    #[test]
    /// NOTE: This test depends on local virtual environment.
    fn environment_python_config() {
        let env = Environment::with_find_venv().unwrap();
        let venv_root = env.python_path().parent().unwrap().parent().unwrap();

        assert_eq!(
            env.python_environment_config().to_string(),
            std::fs::read_to_string(venv_root.join("pyvenv.cfg")).unwrap()
        );
    }

    #[test]
    fn package_display_str() {
        let package = Package::from_str_parts("package", "==", "0.0.0");

        assert_eq!(package.display(), "package==0.0.0");
    }

    #[test]
    fn package_version_operator() {
        let package = Package::from_str_parts("package", "==", "0.0.0");

        assert_eq!(package.version_operator, pep440_rs::Operator::Equal);
    }

    #[test]
    fn package_from_str() {
        let package = Package::from_str("package==0.0.0").unwrap();

        assert_eq!(package.display(), "package==0.0.0");
        assert_eq!(package.name(), "package");
        assert_eq!(
            package.version_operator().to_string(),
            pep440_rs::Operator::Equal.to_string()
        );
        assert_eq!(package.version_str(), "0.0.0");
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
    fn platform_with_find_python() {
        let platform = Platform::with_find_python();

        assert!(platform.python_path.exists())
    }
}
