use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use shared::{
    models::{users::UserAccount, NewUser, User},
    schema::users::dsl,
};

use super::{authentication_status, AuthenticationStatus};
use crate::{errors::MoolahBackendError, Pool};

pub async fn put_user(
    web::Json(new_user): web::Json<NewUser>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    log::debug!("posting registration user form");

    let connection = pool.get()?;

    diesel::insert_into(dsl::users)
        .values(&new_user)
        .execute(&connection)?;

    log::info!("process registration for {}", new_user.username);

    if let Some(username) = id.identity() {
        log::debug!("already logged in for user {} - forgetting", username);
        id.forget();
    }

    id.remember(new_user.username.clone().to_lowercase());
    log::debug!("remebered user session");

    Ok(HttpResponse::Ok().body("processed registration"))
}

#[derive(Deserialize)]
pub struct UserQuery {
    username: String,
}

pub async fn get_user_account(
    query: web::Query<UserQuery>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let requested_username = query.into_inner().username.to_ascii_lowercase();
    match authentication_status(&id, &requested_username) {
        AuthenticationStatus::Matching => {
            let connection = pool.get()?;
            let user: User = dsl::users
                .filter(dsl::username.eq(requested_username))
                .first(&connection)?;

            let account: UserAccount = user.into();

            Ok(HttpResponse::Ok().json(account))
        }
        _ => Ok(HttpResponse::Unauthorized().body("requested username not authenticated for")),
    }
}

pub async fn delete_user(
    query: web::Query<UserQuery>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let requested_username = query.into_inner().username.to_ascii_lowercase();
    match authentication_status(&id, &requested_username) {
        AuthenticationStatus::Matching => {
            let connection = pool.get()?;
            diesel::delete(dsl::users.filter(dsl::username.eq(requested_username)))
                .execute(&connection)?;

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
