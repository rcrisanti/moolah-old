use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::models::{User, UserLoginRequestForm};
use shared::schema::users::dsl::{username, users};

use crate::errors::MoolahBackendError;
use crate::Pool;

pub async fn post_login_request(
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
