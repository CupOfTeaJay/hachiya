//! TODO: Document.

use std::collections::HashMap;
use std::process::{Command, Output};

use bevy::prelude::*;
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use cargo_metadata::{Metadata, Package, PackageName};
use dylib::DynamicLibrary;

use crate::applicator::Applicator;
use crate::exceptions::HachiyaError;
use crate::registrar::Registrar;

/// TODO: Document.
#[derive(Clone, Eq, Hash, PartialEq)]
struct Mod {
    /// TODO: Document.
    package: Package,
}

impl Mod {
    /// TODO: Document.
    pub fn build(&self) -> Output {
        debug!("building {}", self.name());
        Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(&self.package.manifest_path)
            .output()
            .expect("failed to execute cargo build")
    }

    /// TODO: Document.
    pub fn debug(&self) -> Utf8PathBuf {
        self.package
            .manifest_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("target/debug/lib{}.so", self.name()))
    }

    /// TODO: Document.
    pub fn name(&self) -> &PackageName {
        &self.package.name
    }

    /// TODO: Document.
    pub fn new(package: Package) -> Self {
        Mod { package }
    }
}

/// TODO: Document.
struct ModWorkspace {
    /// TODO: Document.
    metadata: Metadata,
}

impl ModWorkspace {
    /// TODO: Document.
    pub fn mods(&self) -> Vec<Mod> {
        self.metadata
            .workspace_members
            .iter()
            .map(|id| Mod::new(self.metadata[id].clone()))
            .collect()
    }

    /// TODO: Document.
    pub fn new(root: Utf8PathBuf) -> Self {
        let metadata: Metadata = MetadataCommand::new()
            .manifest_path(root.join("Cargo.toml"))
            .exec()
            .expect("failed to get cargo metadata");
        ModWorkspace { metadata }
    }
}

/// TODO: Document.
/// TODO: Add ModLibrary for mods without source.
#[derive(Resource)]
pub struct ModRepository {
    /// TODO: Document.
    applicator: Applicator,

    /// TODO: Document.
    registry: HashMap<PackageName, DynamicLibrary>,

    /// TODO: Document.
    root: Utf8PathBuf,

    /// TODO: Document.
    workspace: ModWorkspace,
}

impl ModRepository {
    /// TODO: Document.
    pub fn load(&mut self, world: &mut World) {
        debug!("loading {}", self.root);
        let mut mods: HashMap<Mod, Output> = HashMap::new();
        for _mod in self.workspace.mods() {
            mods.insert(_mod.clone(), _mod.build());
        }
        for (_mod, output) in mods.iter() {
            if output.status.success() {
                debug!("registering {}", _mod.name());
                let lib = DynamicLibrary::open(Some(_mod.debug().as_ref()))
                    .expect("failed to open dylib");
                let hook: fn(&mut Registrar) =
                    unsafe { std::mem::transmute(lib.symbol::<usize>("main").unwrap()) };
                hook(self.applicator.registrar());
                self.applicator.apply(world);
                self.registry.insert(_mod.name().clone(), lib);
            }
        }
    }

    /// TODO: Document.
    pub fn new(root: Utf8PathBuf) -> Result<Self, HachiyaError> {
        if root.is_dir() {
            Ok(ModRepository {
                applicator: Applicator::new(),
                registry: HashMap::new(),
                root: root.clone(),
                workspace: ModWorkspace::new(root.join("src")),
            })
        } else {
            Err(HachiyaError::InitializationError(format!(
                "repository path is not a valid directory: {}",
                root
            )))
        }
    }
}

/// TODO: Justifications.
unsafe impl Send for ModRepository {}
unsafe impl Sync for ModRepository {}
