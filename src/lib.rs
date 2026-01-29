//! TODO: Document.

use std::{env, path::PathBuf};

use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use libloading::Library;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedObjectLoaderError {
    #[error("failed to resolve path to {0}")]
    PathError(String),
}

#[derive(Asset, Default, TypePath)]
pub struct SharedObject {
    _library: Option<Library>,
}

impl SharedObject {
    fn get_assets_root() -> Option<PathBuf> {
        if let Ok(bevy_asset_root) = env::var("BEVY_ASSET_ROOT") {
            Some(PathBuf::from(bevy_asset_root))
        } else if let Ok(cargo_manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            Some(PathBuf::from(cargo_manifest_dir))
        } else if let Some(executable_parent) = env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|parent| parent.to_path_buf()))
        {
            Some(executable_parent)
        } else {
            None
        }
    }
}

impl AssetLoader for SharedObject {
    type Asset = SharedObject;
    type Settings = ();
    type Error = SharedObjectLoaderError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<SharedObject, Self::Error> {
        println!("loading!");
        let asset_path = load_context.path().path();
        if let Some(root) = SharedObject::get_assets_root() {
            let full_path = root.join("assets").join(asset_path);
            Ok(SharedObject {
                _library: unsafe { Some(Library::new(full_path).unwrap()) },
            })
        } else {
            Err(SharedObjectLoaderError::PathError(
                asset_path.display().to_string(),
            ))
        }
    }

    fn extensions(&self) -> &[&str] {
        &["dll", "dylib", "so"]
    }
}

// TODO: Safety.
unsafe impl Send for SharedObject {}
unsafe impl Sync for SharedObject {}
