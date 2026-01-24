//! TODO: Document.

use thiserror::Error;

/// TODO: Document.
#[derive(Debug, Error)]
pub enum HachiyaError {
    #[error("failed to build the repository; {0}")]
    BuildError(String),

    #[error("failed to the repository; {0}")]
    IndexingError(String),

    #[error("failed to initialize Hachiya; {0}")]
    InitializationError(String),

    #[error("failed to load {0}; {1}")]
    LoadError(String, String),
}
