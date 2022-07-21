use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use shared::routes;

mod errors;
mod services;

use errors::MoolahBackendError;
use services::{deltas, login, logout, predictions, user};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type HttpResult = Result<HttpResponse, MoolahBackendError>;

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
            .route(routes::LOGOUT, web::put().to(logout::put_logout))
            .route(
                routes::LOGIN_REQUEST_PASSWORD,
                web::put().to(login::put_request_login_password),
            )
            .route(routes::LOGIN, web::patch().to(login::patch_login))
            .service(
                web::resource(routes::USER)
                    .route(web::get().to(user::get_user_account))
                    .route(web::put().to(user::put_user))
                    .route(web::delete().to(user::delete_user)),
            )
            .service(
                web::resource(routes::PREDICTIONS)
                    .route(web::get().to(predictions::get_predictions))
                    .route(web::put().to(predictions::put_prediction))
                    .route(web::delete().to(predictions::delete_prediction))
                    .route(web::patch().to(predictions::patch_prediction)),
            )
            .service(web::resource(routes::DELTAS).route(web::post().to(deltas::post_delta)))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
