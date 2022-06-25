use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::models::{User, UserLoginRequestForm};
use shared::schema::users::dsl::{email, password, username, users};

use crate::errors::MoolahBackendError;
use crate::Pool;

pub async fn post_login_request_password(
    web::Json(user_form): web::Json<UserLoginRequestForm>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let connection = pool.get()?;

    let user = users
        .filter(username.eq(user_form.username))
        .first::<User>(&connection);

    if let Ok(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::InternalServerError().body("could not retreive user"))
    }
}

pub async fn post_login(
    web::Json(user_form): web::Json<User>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let connection = pool.get()?;

    let user = users
        .filter(username.eq(user_form.username))
        .filter(email.eq(user_form.email))
        .filter(password.eq(user_form.password))
        .first::<User>(&connection);

    if let Ok(user) = user {
        id.remember(user.username);
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().body("incorrect username/email/password combo"))
    }
}
