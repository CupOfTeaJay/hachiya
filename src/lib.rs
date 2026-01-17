//! TODO: Document.

use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Output};

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use cargo_metadata::{Metadata, Package, PackageName};
use dylib::DynamicLibrary;

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
struct ModRepository {
    /// TODO: Document.
    registry: HashMap<PackageName, DynamicLibrary>,

    /// TODO: Document.
    root: Utf8PathBuf,

    /// TODO: Document.
    workspace: ModWorkspace,
}

impl ModRepository {
    /// TODO: Document.
    pub fn load(&mut self) {
        debug!("loading {}", self.root);
        let mut mods: HashMap<Mod, Output> = HashMap::new();
        for _mod in self.workspace.mods() {
            mods.insert(_mod.clone(), _mod.build());
        }
        for (_mod, output) in mods.iter() {
            if output.status.success() {
                debug!("registering {}", _mod.name());
                self.registry.insert(
                    _mod.name().clone(),
                    DynamicLibrary::open(Some(_mod.debug().as_ref()))
                        .expect("failed to open dylib"),
                );
            }
        }
    }

    /// TODO: Document.
    pub fn new(root: Utf8PathBuf) -> Self {
        ModRepository {
            registry: HashMap::new(),
            root: root.clone(),
            workspace: ModWorkspace::new(root.join("src")),
        }
    }
}

/// TODO: Justifications.
unsafe impl Send for ModRepository {}
unsafe impl Sync for ModRepository {}

/// TODO: Document.
fn load_mods(world: &mut World) {
    let messages: usize = SystemState::<MessageReader<LoadMods>>::new(world)
        .get(world)
        .read()
        .count();
    for _ in 0..messages {
        world.resource_scope(|_world: &mut World, mut repository: Mut<ModRepository>| {
            repository.load()
        });
    }
}

/// TODO: Document.
pub struct HachiyaPlugin {
    /// TODO: Document.
    root: Utf8PathBuf,
}

impl HachiyaPlugin {
    /// TODO: Document.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        HachiyaPlugin {
            root: Utf8PathBuf::from_path_buf(root.as_ref().canonicalize().unwrap())
                .expect("path must be valid UTF-8"),
        }
    }
}

impl Plugin for HachiyaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ModRepository::new(self.root.clone()))
            .add_message::<LoadMods>()
            .add_systems(Update, load_mods);
    }
}

/// TODO: Document.
#[derive(Message)]
pub struct LoadMods;
