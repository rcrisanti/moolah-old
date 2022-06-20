use crate::{MoolahBackendError, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::{
    models::{NewUser, UserForm},
    schema,
};

pub async fn post_register(
    user_form: web::Json<UserForm>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    log::debug!("posting registration user form");

    // if let Err(e) = user_form.validate() {
    //     // return Ok(HttpResponse::Ok().body(format!("Registration error: {}", e)));
    //     // return Ok(HttpResponse::SeeOther()
    //     //     .set_header("Location", "/register")
    //     //     .finish());

    //     return register_with_warnings(tera, id, e).await;
    // }

    use schema::users;

    let new_user: NewUser = user_form.into_inner().into();
    let connection = pool.get()?;

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection)?;

    log::info!("process registration for {}", new_user.username);

    Ok(HttpResponse::Ok().body("processed registration"))
}
