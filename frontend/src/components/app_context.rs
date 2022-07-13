use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct ContextData {
    current_username: Option<String>,
}

impl ContextData {
    pub fn new() -> Self {
        ContextData {
            current_username: None,
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.current_username.is_some()
    }

    pub fn current_username(&self) -> Option<String> {
        self.current_username.as_deref().map(|s| s.to_string())
    }

    pub fn login(&mut self, username: String) {
        self.current_username = Some(username.to_lowercase());
    }

    pub fn logout(&mut self) {
        self.current_username = None;
    }
}

pub type AppContext = Rc<RefCell<ContextData>>;
