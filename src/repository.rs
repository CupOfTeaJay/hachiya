//! TODO: Document.

use std::collections::HashMap;
use std::process::{Command, Output, Stdio};

use bevy::prelude::*;
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use cargo_metadata::{CrateType, Metadata, Package, PackageName};
use dylib::DynamicLibrary;

use crate::applicator::Applicator;
use crate::exceptions::HachiyaError;
use crate::registrar::Registrar;

/// TODO: Document.
enum BuildState {
    Building(std::process::Child),
    Built,
    Unbuilt,
}

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
                        self.hook = Some(std::mem::transmute(hook));
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
                match crate_type {
                    CrateType::DyLib => {
                        is_dylib = true;
                        break;
                    }
                    _ => (),
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
            package: package,
        }
    }
}

/// TODO: Document.
#[derive(Resource)]
pub struct ModRepository {
    /// TODO: Document.
    applicator: Applicator,

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

impl ModRepository {
    /// Builds the entire workspace.
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
    pub fn new(root: Utf8PathBuf) -> Result<Self, HachiyaError> {
        if root.is_dir() {
            let extension: &str;
            match std::env::consts::OS {
                "macos" => {
                    extension = ".dylib";
                }
                "windows" => {
                    extension = ".dll";
                }
                "linux" => {
                    extension = ".so";
                }
                _ => {
                    warn!(
                        "assuming a dylib extension of '.so' for unknown OS: {}",
                        std::env::consts::OS
                    );
                    extension = ".so"
                }
            }
            let mut repository: ModRepository = ModRepository {
                applicator: Applicator::new(),
                extension: extension.to_string(),
                members: HashMap::new(),
                metadata: None,
                root: root,
                state: BuildState::Unbuilt,
            };
            if let Err(err) = repository.index() {
                Err(HachiyaError::InitializationError(err.to_string()))
            } else {
                if let Err(err) = repository.build() {
                    Err(HachiyaError::InitializationError(err.to_string()))
                } else {
                    Ok(repository)
                }
            }
        } else {
            Err(HachiyaError::InitializationError(format!(
                "repository path is not a valid directory: {}",
                root
            )))
        }
    }

    /// TODO: Document.
    pub fn poll(&mut self) {
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
unsafe impl Send for ModRepository {}
unsafe impl Sync for ModRepository {}
