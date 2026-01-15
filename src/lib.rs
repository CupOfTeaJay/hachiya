///! TODO: Document.
use std::path::Path;

use cargo_metadata::Metadata;
use cargo_metadata::MetadataCommand;

/// TODO: Document.
#[derive(Debug)]
struct ModLibrary;

/// TODO: Document.
#[derive(Debug)]
struct ModWorkspace {
    /// TODO: Document.
    metadata: Metadata,
}

impl ModWorkspace {
    /// TODO: Document.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let metadata: Metadata = MetadataCommand::new()
            .manifest_path(root.as_ref().join("Cargo.toml"))
            .exec()
            .expect("failed to get cargo metadata");
        ModWorkspace { metadata }
    }
}

/// TODO: Document.
#[derive(Debug)]
pub struct ModRepository {
    /// TODO: Document.
    library: ModLibrary,

    /// TODO: Document.
    workspace: ModWorkspace,
}

impl ModRepository {
    /// TODO: Document.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        ModRepository {
            library: ModLibrary,
            workspace: ModWorkspace::new(root.as_ref().join("src")),
        }
    }
}

