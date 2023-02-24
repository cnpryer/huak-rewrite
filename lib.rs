
/// An API used to interact with some Python project.
pub trait PythonProject {
    /// Get the Python project's metadata file.
    fn metadata_file(&self) -> &PyProjectTOML;
    /// Get the Python project's dependencies.
    fn dependencies(&self) -> &HashMap<String, Vec<Package>>;
    /// Get a specific group of Python dependencies.
    fn dependency_group(&self) -> &Vec<Pacakge>;
}


/// A Python project can be anything from a script to automate some process to a
/// production web application. Projects consist of Python source code and a
/// project-marking metadata file. PEPs provide Python’s ecosystem with standardization
/// and huak leverages them to do many things such as identify your project. See PEP
/// 621. Projects could contain source code used for testing purposes in addition to
/// the application or script source code. To huak, projects are essentially directory
/// trees containing Python code and a pyproject.toml file.
pub struct Project {
    /// A value to indicate the type of the project (app, library, etc.).
    project_type: ProjectType,
    /// The absolute path to the root directory of the project.
    root: PathBuf,
    /// The project file containing metadata about the project.
    metadata_file: PyProjectTOML,
    /// Collections of packages the project depends on.
    dependencies: HashMap<String, Vec<Package>>,
}

/// A project type might indicate if a project is an application-like project or a
/// library-like project.
pub enum ProjectType {
    #[derive(Default)]
    /// Library-like projects are essentially anything that isn’t an application. An
    /// example would be a typical Python package distributed to PyPI.
    Library,
    /// Application-like projects are projects intended to be distributed as an executed
    /// process. Examples would include web applications, automated scripts, etc..
    Application,
}


/// An API used to interact with some Python environment.
pub trait PythonEnvironment {
    /// Create a Python environment on the system.
    fn create();
    /// The absolute path to the Python environment's python interpreter binary.
    fn python_path(&self) -> &PathBuf;
    /// The version of the Python environment's Python interpreter.
    fn python_version(&self) -> &Version;
    /// The absolute path to the Python interpreter used to create the Python
    /// environment.
    fn base_python_path(&self) -> &PathBuf;
    /// The version of the Python interpreter used to create the Python environment.
    fn base_python_version(&self) -> &Version;
    /// The absolute path to the Python environment's executables directory.
    fn executables_directory_path(&self) -> &PathBuf;
    /// The absolute path to the Python environment's site-packages directory.
    fn site_packages_directory_path(&self) -> &PathBuf;
    /// Add a package to the site-packages directory.
    fn add_package_to_site_packages(package: &Package);
    /// Remove a package from the site-packages directory.
    fn remove_package_from_site_packages(package: &Package);
    /// Get a package from the site-packages directory if it is already installed.
    fn find_site_packages_package(name: str) -> &Package;
    /// Get a package's dist info from the site-packages directory if it is there.
    fn find_site_packages_dist_info(name: str) -> &DistInfo;
    /// Check if the Python environment is isolated from any system site-packages
    /// directory.
    fn is_isolated(&self) -> &bool;
    /// Activate the Python environment with the system shell.
    fn activate(&self);
    /// Run a list of values as a command with the Python environment as its context.
    fn run_command_list(&self, list: &[&str]);
    /// Run a string as a command with the Python environment as its context.
    fn run_command_str(&self, string: &str);
}

/// An additional API to interact with a Python environment that isn't isolated.
pub trait IntegratedPythonEnvironment {
    /// The absolute path to the system's executables directory.
    fn base_executables_directory_path(&self) -> &PathBuf;
    /// The absolute path to the system's site-packages directory.
    fn base_site_packages_directory(&self) -> &PathBuf;
    /// Add a package to the system's site-packages directory.
    fn add_package_to_base_site_packages(package: &Package);
    /// Remove a package from the system's site-packages directory.
    fn remove_package_from_base_site_packages(package: &Package);
    /// Get a package from the system's site-packages directory if it is already
    /// installed.
    fn find_base_site_packages_package(name: str) -> Package;
    /// Get a package's dist info from the system's site-packages directory if it is
    /// there.
    fn find_base_site_packages_dist_info(name: str) -> DistInfo;
}

/// Environments can be anything from a model of the system environment to a specific
/// type of Python environment such as a virtual environment.
pub struct Environment {
    /// The environment's configuration data.
    config: EnvironmentConfig,
    /// The absolute path to the environment's Python interpreter.
    python_path: PathBuf,
    /// The absolute path to the environment's executables directory.
    exeutables_directory_path: PathBuf,
    /// The absolute path to the environment's site-packages directory.
    site_packages_directory_path: PathBuf,
    /// The absolute path to the system executables directory.
    base_exeutables_directory_path: PathBuf,
    /// The absolute path to the system site-packages directory.
    base_site_packages_directory_path: PathBuf,
}

/// Data about some environment's configuration. This abstraction is modeled after the
/// pyenv.cfg file used for Python virtual environments.
pub struct EnvironmentConfig {
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
    /// The default shell of the platform.
    shell: todo(),
};