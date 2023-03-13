use error::HuakResult;
use pep440_rs::{Operator as VersionOperator, Version};
use pyproject_toml::PyProjectToml as ProjectToml;
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::RandomState, HashMap},
    fmt::Display,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};
use system::Platform;

mod error;
mod fs;
mod git;
mod operation;
mod shell;
mod system;

const DEFAULT_VENV_NAME: &str = ".venv";
const DEFAULT_PYPROJECT_TOML_CONTENTS: &str = r#"[project]
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

/// An abstraction containing information about the workspace.
pub struct Workspace {
    /// A Python project part of the workspace.
    project: Project,
    /// Environments assigned to the workspace.
    python_environments: HashMap<String, PythonEnvironment>,
    /// A struct to contain useful platform data and objects.
    platform: Platform,
}

impl Workspace {
    /// Create a new workspace.
    pub fn new() -> Workspace {
        Workspace {
            project: Project::new(),
            python_environments: HashMap::new(),
            platform: Platform::new(),
        }
    }

    /// Initialize a workspace from a directory path by attempting to locate
    /// a valid manifest file. If no manifest file is found then the workspace
    /// project root is assumed to be at the current working directory.
    pub fn from_path(dir_path: impl AsRef<Path>) -> HuakResult<Workspace> {
        todo!()
    }

    /// Get the workspace project.
    pub fn project(&self) -> &Project {
        &self.project
    }

    /// Add an environment to the workspace.
    pub fn add_python_environment(&mut self, name: &str, env: PythonEnvironment) -> HuakResult<()> {
        todo!()
    }
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
    pyproject_toml: PyProjectToml,
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

    /// Get the absolute path to the root directory of the project.
    pub fn root(&self) -> &PathBuf {
        todo!()
    }

    /// Get the Python project's pyproject.toml file.
    pub fn pyproject_toml(&self) -> &PyProjectToml {
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

/// A pyproject.toml as specified in PEP 517
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PyProjectToml {
    #[serde(flatten)]
    inner: ProjectToml,
}

impl Default for PyProjectToml {
    fn default() -> Self {
        Self {
            inner: ProjectToml::new(DEFAULT_PYPROJECT_TOML_CONTENTS)
                .expect("could not initilize default pyproject.toml"),
        }
    }
}

impl PyProjectToml {
    /// Create new pyproject.toml data.
    pub fn new() -> PyProjectToml {
        todo!()
    }

    /// Create new pyproject.toml data from a pyproject.toml's path.
    pub fn from_path(path: impl AsRef<Path>) -> HuakResult<PyProjectToml> {
        todo!()
    }

    /// Get the project name.
    pub fn project_name(&self) -> &String {
        todo!()
    }

    /// Set the project name listed in the toml.
    pub fn set_project_name(&mut self, name: &str) {
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

    /// Get the scripts listed in the toml.
    pub fn scripts(&self) -> HashMap<String, String, RandomState> {
        todo!()
    }

    /// Save the toml contents to a filepath.
    pub fn write_file(&self, path: impl AsRef<Path>) -> HuakResult<()> {
        let string = self.to_string_pretty()?;
        Ok(std::fs::write(path, string)?)
    }

    /// Convert the toml struct to a formatted String.
    pub fn to_string_pretty(&self) -> HuakResult<String> {
        Ok(toml_edit::ser::to_string_pretty(&self)?)
    }

    /// Convert the toml to a string as-is.
    pub fn to_string(&self) -> HuakResult<String> {
        Ok(toml_edit::ser::to_string(&self)?)
    }

    /// Check if the project is setup correctly.
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}

pub fn default_pyproject_toml_contents() -> &'static str {
    DEFAULT_PYPROJECT_TOML_CONTENTS
}

/// A PEP-compliant Python environment. Python environments have the following
/// structure:
///  lib
///   └── pythonX.XX
///     └── site-packages
///       ├── some_pkg
///       └── some_pkg-X.X.X.dist-info
/// TODO: More Python environment documentation.
#[derive(Default)]
pub struct PythonEnvironment {
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

impl PythonEnvironment {
    /// Create a new `Environement`.
    pub fn new() -> PythonEnvironment {
        todo!()
    }

    /// Initialize a virtual environment from its absolute path.
    pub fn venv(path: impl AsRef<Path>) -> HuakResult<PythonEnvironment> {
        todo!()
    }

    /// Create a Python virtual environment on the system.
    pub fn create_venv() -> HuakResult<()> {
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

    /// Install a Python package to the environment.
    pub fn install_package(&mut self, package: &Package) -> HuakResult<()> {
        todo!()
    }

    /// Uninstall a Python package from the environment.
    pub fn uninstall_package(&mut self, package_name: &str) -> HuakResult<()> {
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
    pub fn find_site_packages_package(&self, name: &str) -> Option<Package> {
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

    /// Get the pacakge name with its version specifier as a &str.
    pub fn dependency_str(&self) -> &str {
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

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.canonical_name == other.canonical_name
            && self.core_metadata == other.core_metadata
            && self.version == other.version
            && self.version_operator == other.version_operator
            && self.platform_tags == other.platform_tags
    }
}

impl Eq for Package {}

/// Core package metadata.
/// https://packaging.python.org/en/latest/specifications/core-metadata/
#[derive(PartialEq, Eq)]
pub struct PackageMetadata;

/// Tags used to indicate platform compatibility.
/// https://packaging.python.org/en/latest/specifications/platform-compatibility-tags/
#[derive(PartialEq, Eq)]
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

/// Get a hashmap of Python interpreters. Each entry is stored with the interpreter's
/// version as its key and the absolute path the the interpreter as the value.
/// NOTE: This search implementation is inspired by brettcannon/python-launcher
pub fn find_python_interpreter_paths() -> HashMap<Version, PathBuf> {
    let paths = system::env_path_values();
    let interpreters = all_python_interpreters_in_paths(paths);
    interpreters
}

fn all_python_interpreters_in_paths(
    paths: impl IntoIterator<Item = PathBuf>,
) -> HashMap<Version, PathBuf> {
    let mut interpreters = HashMap::new();
    paths.into_iter().for_each(|path| {
        python_version_from_path(&path).map_or((), |version| {
            interpreters.entry(version).or_insert(path);
        })
    });

    interpreters
}

/// Parse a Python interpreter's version from its path if one exists.
fn python_version_from_path(path: impl AsRef<Path>) -> Option<Version> {
    path.as_ref()
        .file_name()
        .or(None)
        .and_then(|raw_file_name| raw_file_name.to_str().or(None))
        .and_then(|file_name| {
            if valid_python_interpreter_file_name(file_name) {
                Version::from_str(&file_name["python".len()..]).ok()
            } else {
                None
            }
        })
}

fn valid_python_interpreter_file_name(file_name: &str) -> bool {
    file_name.len() >= "python3.0".len() && file_name.starts_with("python")
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn workspace_from_path() {
        let dir = tempdir().unwrap().into_path();
        let mock_dir = test_resources_dir_path().join("mock-project");
        fs::copy_dir(&mock_dir, &dir).unwrap();

        let ws = Workspace::from_path(&dir.join("mock-project")).unwrap();
        let project = ws.project();

        assert!(project.project_layout.pyproject_toml_path.exists())
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
        let ptoml = PyProjectToml::from_path(&path).unwrap();

        assert!(ptoml.is_valid());
        assert_eq!(ptoml.project_name(), "mock_project");
        assert_eq!(ptoml.project_version(), "0.0.1");
    }

    #[test]
    fn toml_to_str() {
        let default_toml = PyProjectToml::default();
        let default_toml_str = default_toml.to_string_pretty().unwrap();

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
        let ptoml = PyProjectToml::from_path(path).unwrap();

        assert_eq!(
            ptoml.dependencies().deref(),
            vec!["click==8.1.3", "black==22.8.0", "isort==5.12.0"]
        );
    }

    #[test]
    fn toml_optional_dependencies() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let ptoml = PyProjectToml::from_path(path).unwrap();

        assert_eq!(
            ptoml.optional_dependencey_group("test").deref(),
            vec!["pytest>=6", "mock"]
        );
    }

    #[test]
    fn toml_add_dependency() {
        let path = test_resources_dir_path()
            .join("mock-projet")
            .join("pyproject.toml");
        let mut ptoml = PyProjectToml::from_path(path).unwrap();

        ptoml.add_dependency("test");
        assert_eq!(
            ptoml.to_string_pretty().unwrap(),
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
        let mut ptoml = PyProjectToml::from_path(path).unwrap();

        ptoml.add_optional_dependency("test", "test");
        ptoml.add_optional_dependency("new", "test");
        assert_eq!(
            ptoml.to_string_pretty().unwrap(),
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
        let mut ptoml = PyProjectToml::from_path(path).unwrap();

        ptoml.remove_dependency("isort");
        assert_eq!(
            ptoml.to_string_pretty().unwrap(),
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
        let mut ptoml = PyProjectToml::from_path(path).unwrap();

        ptoml.remove_optional_dependency("test", "mock");
        assert_eq!(
            ptoml.to_string_pretty().unwrap(),
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
    fn python_environment_default() {
        let python_environment = PythonEnvironment::default();

        assert!(python_environment.is_valid());
        assert_eq!(
            python_environment
                .python_path()
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
    /// NOTE: This test depends on local virtual environment.
    fn python_environment_executable_dir_name() {
        let python_environment =
            PythonEnvironment::venv(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".venv"))
                .unwrap();

        assert!(python_environment.python_executables_dir_path().exists());
        #[cfg(unix)]
        assert!(python_environment
            .python_executables_dir_path()
            .join("python")
            .exists());
        #[cfg(windows)]
        assert!(python_environment
            .python_executables_dir_path()
            .join("python.exe")
            .exists());
    }

    #[test]
    /// NOTE: This test depends on local virtual environment.
    fn python_environment_python_config() {
        let python_environment =
            PythonEnvironment::venv(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".venv"))
                .unwrap();
        let venv_root = python_environment
            .python_path()
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        assert_eq!(
            python_environment.python_environment_config().to_string(),
            std::fs::read_to_string(venv_root.join("pyvenv.cfg")).unwrap()
        );
    }

    #[test]
    fn package_display_str() {
        let package = Package::from_str_parts("package", "==", "0.0.0");

        assert_eq!(package.dependency_str(), "package==0.0.0");
    }

    #[test]
    fn package_version_operator() {
        let package = Package::from_str_parts("package", "==", "0.0.0");

        assert_eq!(package.version_operator, pep440_rs::Operator::Equal);
    }

    #[test]
    fn package_from_str() {
        let package = Package::from_str("package==0.0.0").unwrap();

        assert_eq!(package.dependency_str(), "package==0.0.0");
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
    fn python_search() {
        let mut path_vals = system::env_path_values();
        path_vals.insert(0, PathBuf::from("python3.11"));
        path_vals.push(PathBuf::from("python3.10"));
        let path_vals: Vec<String> = path_vals
            .into_iter()
            .map(|path| path.to_str().unwrap().to_string())
            .collect();
        std::env::set_var("PATH", path_vals.join(":"));
        let interpreter_paths = find_python_interpreter_paths();

        assert_eq!(
            interpreter_paths
                .get(&Version::from_str("3.11").unwrap())
                .unwrap()
                .deref(),
            PathBuf::from("python3.11")
        );
        assert_eq!(
            interpreter_paths
                .get(&Version::from_str("3.10").unwrap())
                .unwrap()
                .deref(),
            PathBuf::from("python3.10")
        );
    }
}
