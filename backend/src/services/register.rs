use crate::{MoolahBackendError, Pool};
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::{
    models::{NewUser, UserRegisterForm},
    schema,
};

pub async fn put_register(
    web::Json(user_form): web::Json<UserRegisterForm>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    log::debug!("posting registration user form");

    use schema::users;

    let new_user: Result<NewUser, _> = user_form.try_into();

    match new_user {
        Ok(new_user) => {
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
        Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string())),
    }
}
