#[macro_use]
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use shared::routes;

mod errors;
mod services;

use errors::MoolahBackendError;
use services::post_register;

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
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(&[0; 32])
            //         .name("auth-cookie")
            //         .secure(false),
            // ))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::FormConfig::default().error_handler(|err, req| {
                log::error!("form error: {} ({:?})", &err, req);
                actix_web::Error::from(err)
            }))
            .route(routes::REGISTER, web::post().to(post_register))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}