use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoolahFrontendError {
    #[error("Could not create fully-qualified path")]
    JoinPathError,

    #[error("Could not get current window")]
    WebSysError,

    #[error("Error compiling regex")]
    RegexError(#[from] regex::Error),
}
