use std::{cell::RefCell, rc::Rc};

use gloo_storage::{SessionStorage, Storage};
use serde::Serialize;

use crate::MoolahFrontendError;

pub type AppContext = Rc<RefCell<ContextData>>;

#[derive(Clone, Debug, PartialEq)]
pub struct ContextData {
    username: Option<String>,
}

impl ContextData {
    pub fn new() -> Self {
        ContextData { username: None }
    }

    pub fn is_logged_in(&mut self) -> bool {
        if self.username.is_some() {
            true
        } else {
            self.username = identity_recall();
            self.username.is_some()
        }
    }

    pub fn username(&mut self) -> Option<&str> {
        if !self.is_logged_in() {
            None
        } else {
            self.username.as_deref()
        }
    }

    pub fn login(&mut self, username: String) -> Result<(), MoolahFrontendError> {
        let username = username.to_ascii_lowercase();
        identity_remember(&username)?;
        self.username = Some(username);
        Ok(())
    }

    pub fn logout(&mut self) {
        self.username = None;
        identity_forget();
    }
}

const SESSION_STORAGE_ID_NAME: &str = "moolah-username";

fn identity_remember<T: Serialize>(username: T) -> Result<(), MoolahFrontendError> {
    SessionStorage::set(SESSION_STORAGE_ID_NAME, username)?;

    log::trace!("remebered identity in session storage");

    Ok(())
}

fn identity_recall() -> Option<String> {
    log::trace!("recalling identity from session storage");

    SessionStorage::get(SESSION_STORAGE_ID_NAME).ok()
}

fn identity_forget() {
    log::trace!("forgetting identity from session storage");

    SessionStorage::delete(SESSION_STORAGE_ID_NAME)
}
