//! TODO: Document.

use std::path::Path;
use std::process::{Command, Output};

use bevy::prelude::*;
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use cargo_metadata::{Metadata, Package, PackageName};

/// TODO: Document.
struct Mod {
    /// TODO: Document.
    package: Package,
}

impl Mod {
    /// TODO: Document.
    pub fn build(&self) -> Output {
        Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(&self.package.manifest_path)
            .output()
            .expect("failed to execute cargo build")
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
    workspace: ModWorkspace,
}

impl ModRepository {
    /// TODO: Document.
    pub fn mods(&self) -> Vec<Mod> {
        self.workspace.mods()
    }

    /// TODO: Document.
    pub fn new(root: Utf8PathBuf) -> Self {
        ModRepository {
            workspace: ModWorkspace::new(root.join("src")),
        }
    }
}

fn load_mods(world: &mut World) {
    world.resource_scope(|_world: &mut World, repository: Mut<ModRepository>| {
        for _mod in repository.mods() {
            let output = _mod.build();
            println!("built mod {} with {}", _mod.name(), output.status);
        }
    });
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
            root: Utf8PathBuf::from_path_buf(root.as_ref().to_path_buf())
                .expect("path must be valid UTF-8"),
        }
    }
}

impl Plugin for HachiyaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ModRepository::new(self.root.clone()))
            .add_systems(Startup, load_mods);
    }
}

// /// TODO: Document.
// #[derive(Event)]
// pub struct LoadModsEvent;
