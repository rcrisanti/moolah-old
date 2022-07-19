use actix_identity::Identity;

pub mod account;
pub mod login;
pub mod logout;
pub mod predictions;
pub mod register;

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
