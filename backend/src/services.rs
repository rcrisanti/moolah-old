use actix_identity::Identity;

mod account;
mod login;
mod logout;
mod predictions;
mod register;

pub use account::{delete_account, get_account};
pub use login::{post_login, post_login_request_password};
pub use logout::post_logout;
pub use predictions::{get_predictions, post_prediction};
pub use register::post_register;

enum AuthenticationStatus {
    Matching,
    Mismatched,
    Unauthorized,
}

fn is_authenticated_user(id: &Identity, username: &str) -> AuthenticationStatus {
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
