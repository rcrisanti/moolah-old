use gloo_storage::{SessionStorage, Storage};
use serde::Serialize;

use crate::MoolahFrontendError;

const SESSION_STORAGE_ID_NAME: &str = "moolah-username";

pub fn identity_remember<T: Serialize>(username: T) -> Result<(), MoolahFrontendError> {
    SessionStorage::set(SESSION_STORAGE_ID_NAME, username)?;

    log::debug!("remebered identity");

    Ok(())
}

pub fn identity_recall() -> Option<String> {
    log::debug!("recalling identity");

    SessionStorage::get(SESSION_STORAGE_ID_NAME).ok()
}

pub fn identity_forget() {
    log::debug!("forgetting identity");

    SessionStorage::delete(SESSION_STORAGE_ID_NAME)
}
