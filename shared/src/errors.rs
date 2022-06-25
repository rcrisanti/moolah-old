use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoolahSharedError {
    #[error("Error validating form")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Error hashing password")]
    PasswordHashError(#[from] argon2::password_hash::Error),
}
