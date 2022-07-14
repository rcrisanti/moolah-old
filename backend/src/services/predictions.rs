use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::{insert_into, prelude::*};
use shared::models::{DbDelta, Delta, NewPrediction, Prediction, PredictionWithDeltas};
use shared::schema::predictions::dsl;

use super::is_authenticated;
use crate::{errors::MoolahBackendError, Pool};

pub async fn get_predictions(
    path: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let username = path.into_inner();

    if is_authenticated(&id, &username) {
        let connection = pool.get()?;

        let preds = dsl::predictions
            .filter(dsl::username.eq(username))
            .load::<Prediction>(&connection)?;

        let deltas = DbDelta::belonging_to(&preds)
            .load::<DbDelta>(&connection)?
            .grouped_by(&preds);

        let full_preds = preds
            .into_iter()
            .zip(deltas)
            .map(|(pred, deltas)| {
                (
                    pred,
                    deltas
                        .into_iter()
                        .map(|d| Delta::try_from(d).expect("could not convert to delta"))
                        .collect::<Vec<_>>(),
                )
                    .into()
            })
            .collect::<Vec<PredictionWithDeltas>>();

        Ok(HttpResponse::Ok().json(full_preds))
    } else {
        log::debug!("user is not authorized to get predictions for this user");
        Ok(HttpResponse::Unauthorized().finish())
    }
}

pub async fn post_prediction(
    path: web::Path<String>,
    web::Json(prediction): web::Json<NewPrediction>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let username = path.into_inner();

    if !is_authenticated(&id, &username) || username != prediction.username() {
        log::debug!("user is not authorized to post this prediction");
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let connection = pool.get()?;

    let prediction: PredictionWithDeltas = insert_into(dsl::predictions)
        .values(prediction)
        // .on_conflict_do_nothing()
        .get_result::<Prediction>(&connection)?
        .into();

    log::debug!("completed insert of 1 rows");
    Ok(HttpResponse::Ok().json(prediction))
}

pub async fn delete_prediction(
    path: web::Path<String>,
    web::Json(prediction): web::Json<Prediction>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let username = path.into_inner();

    if !is_authenticated(&id, &username) || username != prediction.username() {
        log::debug!("user is not authorized to delete this prediction");
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let connection = pool.get()?;

    let n_deleted_rows = diesel::delete(
        dsl::predictions
            .filter(dsl::id.eq(prediction.id()))
            .filter(dsl::username.eq(prediction.username()))
            .filter(dsl::name.eq(prediction.name())),
    )
    .execute(&connection)?;

    log::info!("deleted {} prediction", n_deleted_rows);

    Ok(HttpResponse::Ok().finish())
}
