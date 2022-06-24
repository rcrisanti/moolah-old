use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoolahBackendError {
    #[error("Environment error")]
    EnvironmentError(#[from] std::env::VarError),

    #[error("R2D2 error")]
    R2D2Error(#[from] r2d2::Error),

    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),
    // #[error("Argonautica error")]
    // ArgonauticaError(#[from] argonautica::Error),
}

impl ResponseError for MoolahBackendError {}
