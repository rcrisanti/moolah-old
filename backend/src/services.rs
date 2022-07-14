use actix_identity::Identity;

mod account;
mod login;
mod logout;
mod predictions;
mod register;

pub use account::{delete_account, get_account};
pub use login::{post_login, post_login_request_password};
pub use logout::post_logout;
pub use predictions::{delete_prediction, get_predictions, post_prediction};
pub use register::post_register;

#[derive(PartialEq)]
enum AuthenticationStatus {
    Matching,
    Mismatched,
    Unauthorized,
}

fn authentication_status(id: &Identity, username: &str) -> AuthenticationStatus {
    if let Some(auth_username) = id.identity() {
        if auth_username.to_lowercase() == username.to_lowercase() {
            AuthenticationStatus::Matching
        } else {
            AuthenticationStatus::Mismatched
        }
    } else {
        AuthenticationStatus::Unauthorized
    }
}

fn is_authenticated(id: &Identity, username: &str) -> bool {
    authentication_status(id, username) == AuthenticationStatus::Matching
}
