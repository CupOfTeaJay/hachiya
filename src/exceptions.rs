//! TODO: Document.

use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HachiyaError {
    #[error("failed to get 'main' symbol from shared object; {0}")]
    InvalidHook(#[from] Box<dyn Error + Send + Sync>),

    #[error("failed to load shared object;")]
    LoadFailure(#[from] libloading::Error),

    #[error("failed to resolve the base-path of the assets directory")]
    UnresolvedAssets,
}
