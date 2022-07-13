use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::{insert_into, prelude::*};
use shared::models::{DbDelta, Delta, NewPrediction, Prediction, PredictionWithDeltas};
use shared::schema::predictions::dsl;

use super::{is_authenticated_user, AuthenticationStatus};
use crate::{errors::MoolahBackendError, Pool};

pub async fn get_predictions(
    requested_username: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    log::info!("getting predictions");

    let requested_username = requested_username.into_inner().to_lowercase();
    match is_authenticated_user(&id, &requested_username) {
        AuthenticationStatus::Matching => {
            let connection = pool.get()?;

            let preds = dsl::predictions
                .filter(dsl::username.eq(requested_username))
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
        }
        _ => Ok(HttpResponse::Unauthorized().finish()),
    }
}

pub async fn post_prediction(
    path: web::Path<String>,
    web::Json(prediction): web::Json<NewPrediction>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let username = path.into_inner();

    match is_authenticated_user(&id, &username) {
        AuthenticationStatus::Matching => {
            if username == prediction.username {
                log::debug!("is authorized user");
                let connection = pool.get()?;

                let prediction: PredictionWithDeltas = insert_into(dsl::predictions)
                    .values(prediction)
                    // .on_conflict_do_nothing()
                    .get_result::<Prediction>(&connection)?
                    .into();

                log::debug!("completed insert of 1 rows");
                Ok(HttpResponse::Ok().json(prediction))
            } else {
                log::debug!("is not authorized user");
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        _ => {
            log::debug!("is authorized user");
            Ok(HttpResponse::Unauthorized().finish())
        }
    }
}
