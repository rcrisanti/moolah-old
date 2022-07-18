use actix_identity::Identity;
use actix_web::HttpResponse;

use crate::MoolahBackendError;

pub async fn put_logout(id: Identity) -> Result<HttpResponse, MoolahBackendError> {
    id.forget();
    log::debug!("forgot user id cookie session");

    Ok(HttpResponse::Ok().finish())
}
