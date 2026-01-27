//! TODO: Document.

use std::collections::HashMap;
use std::process::{Command, Stdio};

use bevy::prelude::*;
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use cargo_metadata::{CrateType, Metadata, Package, PackageName};
use dylib::DynamicLibrary;

use crate::applicator::Applicator;
use crate::exceptions::HachiyaError;
use crate::plugin::HachiyaPlugin;
use crate::registrar::Registrar;

/// TODO: Document.
struct Mod {
    /// TODO: Document.
    hook: Option<fn(&mut Registrar)>,

    /// TODO: Document.
    library: Option<DynamicLibrary>,

    /// TODO: Document.
    package: Package,
}

impl Mod {
    /// TODO: Document.
    fn load(&mut self, path: Utf8PathBuf) -> Result<&mut Self, HachiyaError> {
        match DynamicLibrary::open(Some(path.as_ref())) {
            Ok(library) => unsafe {
                match library.symbol::<usize>("main") {
                    Ok(hook) => {
                        self.hook =
                            Some(std::mem::transmute::<*mut usize, fn(&mut Registrar)>(hook));
                        self.library = Some(library);
                        Ok(self)
                    }
                    Err(err) => Err(HachiyaError::LoadError(self.name().to_string(), err)),
                }
            },
            Err(err) => Err(HachiyaError::LoadError(self.name().to_string(), err)),
        }
    }

    /// TODO: Document.
    pub fn is_dylib(&self) -> bool {
        let mut is_dylib: bool = false;
        if let Some(target) = self.package.targets.first() {
            for crate_type in target.crate_types.iter() {
                if crate_type == &CrateType::DyLib {
                    is_dylib = true;
                    break;
                }
            }
        } else {
            error!("no targets specified for repository member {}", self.name());
        }
        is_dylib
    }

    /// TODO: Document.
    pub fn hook(&self, registrar: &mut Registrar) {
        if let Some(hook) = self.hook {
            hook(registrar);
        }
    }

    /// TODO: Document.
    pub fn load_debug(
        &mut self,
        root: Utf8PathBuf,
        extension: &str,
    ) -> Result<&mut Self, HachiyaError> {
        self.load(root.join(format!("target/debug/lib{}{}", self.name(), extension)))
    }

    /// TODO: Document.
    pub fn _load_release(
        &mut self,
        root: Utf8PathBuf,
        extension: &str,
    ) -> Result<&mut Self, HachiyaError> {
        self.load(root.join(format!("target/release/lib{}{}", self.name(), extension)))
    }

    /// TODO: Document.
    pub fn name(&self) -> &PackageName {
        &self.package.name
    }

    /// TODO: Document.
    pub fn new(package: Package) -> Self {
        Mod {
            hook: None,
            library: None,
            package,
        }
    }
}

/// TODO: Document.
pub enum BuildState {
    Building(std::process::Child),
    Built,
    Unbuilt,
}

/// TODO: Document.
#[derive(Clone, Debug)]
pub enum BuildTarget {
    Debug,
    Release,
}

/// TODO: Document.
#[derive(Resource)]
pub struct Repository {
    /// TODO: Document.
    _applicator: Applicator,

    /// TODO: Document.
    extension: String,

    /// TODO: Document.
    members: HashMap<PackageName, Mod>,

    /// TODO: Document.
    metadata: Option<Metadata>,

    /// TODO: Document.
    root: Utf8PathBuf,

    /// TODO: Document.
    state: BuildState,
}

impl Repository {
    /// Determines what the expected shared-library extension should be
    /// according to the host operating-system.
    ///
    /// If the OS is not `macos`, `windows`, or `linux`, then just assume an
    /// extension of `.so`.
    fn determine_dylib_extension() -> String {
        let extension: &str = match std::env::consts::OS {
            "macos" => ".dylib",
            "windows" => ".dll",
            "linux" => ".so",
            _ => {
                warn!("unknown OS: {}", std::env::consts::OS);
                ".so"
            }
        };
        info!(
            "expecting '{}' shared-library extensions for OS: {}",
            extension,
            std::env::consts::OS
        );
        extension.to_string()
    }

    /// Determines what the root directory of this [`Repository`] should be
    /// according to the current execution environment.
    ///
    /// This only runs if the user did not designate a custom path. If a path
    /// was specified, then there are two possible fallbacks:
    ///   1. If the `CARGO_MANIFEST_DIR` environment variable is set, then
    ///      assume a development context and look for a repository adjacent to
    ///      the project's `Cargo.toml` under a `mods/` directory
    ///   2. Otherwise, assume a deployment context and expect the repository
    ///      to be located alongside the executable under a `mods/` directory
    ///
    /// # Errors
    ///
    /// Returns a [`HachiyaError::InitializationError`] if:
    ///   * The executable's path cannot be determined
    ///   * Or the executable's parent directory could not be determined
    ///   * Or the predicted repository location is not a valid directory
    fn resolve_root() -> Result<Utf8PathBuf, HachiyaError> {
        let mut root: Utf8PathBuf;
        if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
            info!("resolving path to repository via CARGO_MANIFEST_DIR");
            root = Utf8PathBuf::from(dir);
        } else {
            info!("resolving path to repository via std::env::current_exe()");
            match std::env::current_exe() {
                Ok(exe) => {
                    if let Some(parent) = exe.parent() {
                        root = Utf8PathBuf::from(parent.to_string_lossy().into_owned());
                    } else {
                        return Err(HachiyaError::InitializationError(format!(
                            "repository could not be resolved from executable: {} has no parent",
                            exe.display()
                        )));
                    }
                }
                Err(err) => {
                    return Err(HachiyaError::InitializationError(format!(
                        "repository path could not be resolved from executable: {}",
                        err
                    )));
                }
            }
        }
        root.push("mods");
        if root.is_dir() {
            Ok(root)
        } else {
            Err(HachiyaError::InitializationError(format!(
                "expected repository path is not a valid directory: {}",
                root
            )))
        }
    }

    /// TODO: Document.
    fn _resolve_sdk() {
        todo!()
    }

    /// Validates the
    /// [`HachiyaPlugin::repository`](crate::HachiyaPlugin::repository) field,
    /// if one was supplied by the user.
    ///
    /// # Errors
    ///
    /// Returns a [`HachiyaError::InitializationError`] if:
    ///   * The given `root` does not exist
    ///   * Or the given `root` is not a directory
    fn validate_user_root(root: &String) -> Result<Utf8PathBuf, HachiyaError> {
        info!("resolving path to repository via user-designated root");
        let root: Utf8PathBuf = Utf8PathBuf::from(root);
        if root.is_dir() {
            Ok(root)
        } else {
            Err(HachiyaError::InitializationError(format!(
                "designated repository path is not a valid directory: {}",
                root
            )))
        }
    }

    /// Builds the entire workspace asynchronously.
    pub fn build(&mut self) -> Result<(), HachiyaError> {
        info!("building repository");
        match &self.state {
            BuildState::Building(process) => Err(HachiyaError::BuildError(format!(
                "child process {} is already running",
                process.id()
            ))),
            _ => {
                if let Ok(process) = Command::new("cargo")
                    .arg("build")
                    .arg("--manifest-path")
                    .arg(self.root.join("Cargo.toml"))
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    self.state = BuildState::Building(process);
                    Ok(())
                } else {
                    Err(HachiyaError::BuildError(
                        "failed to spawn build process".to_string(),
                    ))
                }
            }
        }
    }

    /// Deserializes the workspace manifest associated with this repository.
    /// This called whenever the `Cargo.toml` is changed, for example, after
    /// adding another member to the workspace.
    pub fn index(&mut self) -> Result<(), HachiyaError> {
        if let Ok(metadata) = MetadataCommand::new()
            .manifest_path(self.root.join("Cargo.toml"))
            .exec()
        {
            for id in metadata.workspace_members.iter() {
                let member = Mod::new(metadata[id].clone());
                self.members.insert(member.name().clone(), member);
            }
            self.metadata = Some(metadata);
            Ok(())
        } else {
            self.members.clear();
            self.metadata = None;
            Err(HachiyaError::IndexingError(format!(
                "could not find or deserialize a Cargo.toml under {}",
                self.root
            )))
        }
    }

    /// TODO: Document.
    pub fn load_all(&mut self, _world: &mut World) -> Result<(), HachiyaError> {
        match self.state {
            BuildState::Built => {
                info!("loading repository");
                let mut registrar: Registrar = Registrar::new();
                for member in self.members.values_mut() {
                    if member.is_dylib() {
                        match member.load_debug(self.root.clone(), &self.extension) {
                            Ok(member) => {
                                debug!("loading {}", member.name());
                                member.hook(&mut registrar);
                            }
                            Err(err) => error!("{}", err),
                        }
                    }
                }
                Ok(())
            }
            _ => Err(HachiyaError::LoadError(
                "mods".to_string(),
                "the repository is unbuilt".to_string(),
            )),
        }
    }

    /// TODO: Document.
    pub fn new(plugin: &HachiyaPlugin) -> Result<Self, HachiyaError> {
        let mut repository: Repository = Repository {
            _applicator: Applicator::new(),
            extension: Repository::determine_dylib_extension(),
            members: HashMap::new(),
            metadata: None,
            root: if let Some(user_root) = &plugin.repository_path {
                Repository::validate_user_root(user_root)?
            } else {
                Repository::resolve_root()?
            },
            state: BuildState::Unbuilt,
        };
        if let Err(err) = repository.index() {
            Err(HachiyaError::InitializationError(err.to_string()))
        } else {
            Ok(repository)
        }
    }

    /// Gets a reference to the [`BuildState`] of this [`Repository`].
    ///
    /// This may be used to get a read-only handle of the underlying build
    /// process, if one is in-progress.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use hachiya::{BuildState, Repository};
    ///
    /// // Fetches the mod repository and checks it's build state.
    /// fn check_build_state(repository: Res<Repository>) {
    ///     match repository.state() {
    ///         BuildState::Unbuilt => println!("the repository is not built"),
    ///         BuildState::Building(process) => {
    ///             let pid: u32 = process.id();
    ///             println!("process {} is building the repository", pid);
    ///         },
    ///         BuildState::Built => println!("the repository is built")
    ///     }
    /// }
    ///
    /// ```
    pub fn state(&self) -> &BuildState {
        &self.state
    }

    /// Transitions this [`Repository`] to a new [`BuildState`], depending on
    /// the outcome of the previous build.
    ///
    /// If all of the underlying workspace members compiled successfully then
    /// this repository will be considered [`BuildState::Built`], and may
    /// therefore load mods. If a build process was unsuccessful, or if one was
    /// never started, then the repository will be in the
    /// [`BuildState::Unbuilt`] state.
    ///
    /// By default, the [`HachiyaPlugin`] schedule's a helper system in Bevy's
    /// `Update` schedule that continuously call's this method, so it is
    /// unlikely one would ever want to invoke it manually.
    pub fn update(&mut self) {
        if let BuildState::Building(handle) = &mut self.state {
            match handle.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        info!("successfully built repository");
                        self.state = BuildState::Built;
                    } else {
                        error!("failed to build repository: {}", status);
                        self.state = BuildState::Unbuilt;
                    }
                }
                Err(err) => {
                    error!("failed to poll build process: {}", err);
                    self.state = BuildState::Unbuilt;
                }
                _ => (),
            }
        }
    }
}

/// TODO: Justifications.
unsafe impl Send for Repository {}
unsafe impl Sync for Repository {}
