use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::{
    models::{users::UserAccount, User},
    schema::users::dsl::{username, users},
};

use crate::{errors::MoolahBackendError, Pool};

pub async fn get_account(
    requested_username: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let requested_username = requested_username.into_inner().to_lowercase();
    if Some(requested_username.clone()) == id.identity() {
        let connection = pool.get()?;
        let user: User = users
            .filter(username.eq(requested_username))
            .first(&connection)?;

        let account: UserAccount = user.into();

        Ok(HttpResponse::Ok().json(account))
    } else {
        Ok(HttpResponse::Unauthorized().body("requested username not authenticated for"))
    }
}

pub async fn delete_account(
    requested_username: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let requested_username = requested_username.into_inner().to_lowercase();
    if let Some(auth_username) = id.identity() {
        if auth_username == requested_username {
            let connection = pool.get()?;
            diesel::delete(users.filter(username.eq(auth_username))).execute(&connection)?;

            id.forget();
            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::Unauthorized().body("not authorized to delete this account"))
        }
    } else {
        Ok(HttpResponse::Unauthorized().body("not logged in"))
    }
}
