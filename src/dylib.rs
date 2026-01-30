//! TODO: Document.

use std::{
    env,
    path::{Path, PathBuf},
};

use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use libloading::Library;
use stabby::libloading::StabbyLibrary;

use crate::dynapp::DynamicApp;
use crate::exceptions::HachiyaError;

/// The expected function signature of the entry-point exported by a [`Dylib`].
type DynamicAppHook = extern "C" fn(stabby::boxed::Box<DynamicApp>);

#[derive(Asset, Default, TypePath)]
pub struct Dylib {
    /// The dynamic library's [`DynamicAppHook`] entry-point.
    _hook: Option<DynamicAppHook>,

    /// The underlying handle of the dynamic library.
    _library: Option<Library>,
}

impl Dylib {
    /// Attempts to import a symbol called `main` with the [`DynamicAppHook`]
    /// function signature from a shared-object.
    ///
    /// # Errors
    ///
    /// * [`HachiyaError::InvalidHook`] if the symbol could not be resolved or
    ///   an ABI mismatch is suspected
    fn get_main(library: &Library) -> Result<DynamicAppHook, HachiyaError> {
        Ok(*unsafe { library.get_stabbied::<DynamicAppHook>(b"main")? })
    }

    /// Attempts to load a shared-object from disk via the `libloading` crate.
    ///
    /// # Errors
    ///
    /// * [`HachiyaError::LoadFailure`] if the shared-object could not be
    ///   loaded from disk or opened
    fn load_library(path: &PathBuf) -> Result<Library, HachiyaError> {
        Ok(unsafe { Library::new(path)? })
    }

    /// Attempts to construct a new [`Dylib`] instance from a shared-object
    /// located at the given `path`.
    ///
    /// # Errors
    ///
    /// * [`HachiyaError::LoadFailure`] if the shared-object could not be
    ///   initialized
    /// * [`HachiyaError::InvalidHook`] if a symbol named `main` with the
    ///   [`DynamicAppHook`] signature was not exported by the shared-object, or
    ///   if an ABI mismatch was detected
    fn new(path: &Path) -> Result<Self, HachiyaError> {
        let library: Library =
            Self::load_library(&Self::resolve_assets()?.join("assets").join(path))?;
        Ok(Dylib {
            _hook: Some(Self::get_main(&library)?),
            _library: Some(library),
        })
    }

    /// Determines what the base-path of the user's assets directory should be.
    ///
    /// Three different scenarios are possible, depending on the context:
    ///   1. If the `BEVY_ASSET_ROOT` environment variable is set, then assume
    ///      the assets are located here
    ///   2. Otherwise fallback to using the `CARGO_MANIFEST_DIR` environment
    ///      variable
    ///   3. Otherwise presume that they are adjacent to the currently running
    ///      executable
    ///
    /// # Errors
    ///
    /// * [`HachiyaError::UnresolvedAssets`] if the expected path to the
    ///   assets could not be inferred by any of the means listed above.
    fn resolve_assets() -> Result<PathBuf, HachiyaError> {
        if let Ok(bevy_asset_root) = env::var("BEVY_ASSET_ROOT") {
            Ok(PathBuf::from(bevy_asset_root))
        } else if let Ok(cargo_manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            Ok(PathBuf::from(cargo_manifest_dir))
        } else if let Some(executable_parent) = env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|parent| parent.to_path_buf()))
        {
            Ok(executable_parent)
        } else {
            Err(HachiyaError::UnresolvedAssets)
        }
    }
}

impl AssetLoader for Dylib {
    type Asset = Dylib;
    type Settings = ();
    type Error = HachiyaError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Dylib, HachiyaError> {
        Dylib::new(load_context.path().path())
    }

    fn extensions(&self) -> &[&str] {
        &["dll", "dylib", "so"]
    }
}

// TODO: Safety.
unsafe impl Send for Dylib {}
unsafe impl Sync for Dylib {}
