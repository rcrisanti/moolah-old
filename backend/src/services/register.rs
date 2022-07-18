use crate::{MoolahBackendError, Pool};
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::{models::NewUser, schema};

pub async fn put_register(
    web::Json(new_user): web::Json<NewUser>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    log::debug!("posting registration user form");

    use schema::users;

    let connection = pool.get()?;

    diesel::insert_into(users::table)
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
