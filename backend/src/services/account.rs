use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::{
    models::{users::UserAccount, User},
    schema::users::dsl::{username, users},
};

use super::{is_authenticated_user, AuthenticationStatus};
use crate::{errors::MoolahBackendError, Pool};

pub async fn get_account(
    requested_username: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let requested_username = requested_username.into_inner().to_lowercase();
    match is_authenticated_user(&id, &requested_username) {
        AuthenticationStatus::Matching => {
            let connection = pool.get()?;
            let user: User = users
                .filter(username.eq(requested_username))
                .first(&connection)?;

            let account: UserAccount = user.into();

            Ok(HttpResponse::Ok().json(account))
        }
        _ => Ok(HttpResponse::Unauthorized().body("requested username not authenticated for")),
    }
}

pub async fn delete_account(
    requested_username: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let requested_username = requested_username.into_inner().to_lowercase();
    match is_authenticated_user(&id, &requested_username) {
        AuthenticationStatus::Matching => {
            let connection = pool.get()?;
            diesel::delete(users.filter(username.eq(requested_username))).execute(&connection)?;

            id.forget();
            Ok(HttpResponse::Ok().finish())
        }
        AuthenticationStatus::Mismatched => {
            Ok(HttpResponse::Unauthorized().body("not authorized to delete this account"))
        }
        AuthenticationStatus::Unauthorized => {
            Ok(HttpResponse::Unauthorized().body("not logged in"))
        }
    }
}
