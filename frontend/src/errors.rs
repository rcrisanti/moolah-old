use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoolahFrontendError {
    #[error("could not create fully-qualified path")]
    JoinPathError,

    #[error("could not get current window")]
    WebSysError,

    #[error("error compiling regex")]
    RegexError(#[from] regex::Error),

    #[error("error with local or session web storage")]
    StorageError(#[from] gloo_storage::errors::StorageError),
}

#[derive(Error, Debug, Clone)]
pub enum InternalResponseError {
    #[error("unauthorized to complete requested action")]
    Unauthorized,

    #[error("you already have a {0} with the same {1}")]
    UniqueConstraintViolation(&'static str, String),

    #[error("unable to receive {0} from response: {1}")]
    ResponseAwaitError(&'static str, String),

    #[error("{0}")]
    Other(String),
}
