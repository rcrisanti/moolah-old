use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoolahFrontendError {
    #[error("Could not create fully-qualified path")]
    JoinPathError,

    #[error("Could not get current window")]
    WebSysError,

    #[error("Could not store username in session storage")]
    GlooStorageError(#[from] gloo_storage::errors::StorageError),

    #[error("Error compiling regex")]
    RegexError(#[from] regex::Error),
}
