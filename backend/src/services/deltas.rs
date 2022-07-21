use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use shared::models::NewDbDelta;
use shared::schema::{deltas, predictions};

use crate::{services::is_authenticated, HttpResult, Pool};

pub async fn post_delta(
    path: web::Path<String>,
    web::Json(db_delta): web::Json<NewDbDelta>,
    id: Identity,
    pool: web::Data<Pool>,
) -> HttpResult {
    let username = path.into_inner();

    let connection = pool.get()?;

    let prediction_user = predictions::table
        .filter(predictions::dsl::id.eq(db_delta.prediction_id()))
        .select(predictions::dsl::username)
        .get_result::<String>(&connection)?;

    if !is_authenticated(&id, &username) || username != prediction_user {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    diesel::insert_into(deltas::dsl::deltas)
        .values(&db_delta)
        .execute(&connection)?;

    Ok(HttpResponse::Ok().finish())
}

// pub async fn get_delta() {}

// pub async fn patch_delta() {}

// pub async fn delete_delta() {}
