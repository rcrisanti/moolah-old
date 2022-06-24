use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MoolahFrontendError {
    #[error("Could not create fully-qualified path")]
    JoinPathError,

    #[error("Could not get current window")]
    WebSysError,
}
