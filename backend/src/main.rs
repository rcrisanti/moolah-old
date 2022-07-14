// #[macro_use]
// extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use shared::routes;

mod errors;
mod services;

use errors::MoolahBackendError;
use services::{
    delete_account, delete_prediction, get_account, get_predictions, post_login,
    post_login_request_password, post_logout, post_prediction, post_register,
};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable not set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create Postgres pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .same_site(SameSite::None),
            ))
            .app_data(web::Data::new(pool.clone()))
            .route(routes::REGISTER, web::post().to(post_register))
            .route(routes::LOGOUT, web::post().to(post_logout))
            .route(
                routes::LOGIN_REQUEST_PASSWORD,
                web::post().to(post_login_request_password),
            )
            .route(routes::LOGIN, web::post().to(post_login))
            .service(
                web::resource(routes::ACCOUNT)
                    .route(web::get().to(get_account))
                    .route(web::delete().to(delete_account)),
            )
            .service(
                web::resource(routes::PREDICTIONS)
                    .route(web::get().to(get_predictions))
                    .route(web::post().to(post_prediction))
                    .route(web::delete().to(delete_prediction)),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
