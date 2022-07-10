use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoolahSharedError {
    #[error("error validating form")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("error hashing password")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    // #[error("Attempted to convert between incompatible Delta types ({from} -> {to})")]
    // DeltaConversionError { from: String, to: String },
    #[error("unable to convert between Delta and DbDelta: ({0})")]
    DeltaConversionError(&'static str),

    #[error("unable to seriealize Delta: {0}")]
    DeltaSerializationError(&'static str),

    #[error("unable to deseriealize Delta: {0}")]
    DeltaDeserializationError(&'static str),

    #[error("delta repetition error: {0}")]
    RepetitionError(&'static str),
}
