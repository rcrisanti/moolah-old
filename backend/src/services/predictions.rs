use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::{insert_into, prelude::*};
use shared::models::predictions::PredictionWithDeltas;
// use shared::models::predictions::PredictionWithDeltas;
use shared::models::{Delta, DeltaDate, DeltaWithDates, NewPrediction, Prediction};
use shared::schema::predictions::dsl;

use super::{is_authenticated_user, AuthenticationStatus};
use crate::{errors::MoolahBackendError, Pool};

pub async fn get_predictions(
    requested_username: web::Path<String>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, MoolahBackendError> {
    let requested_username = requested_username.into_inner().to_lowercase();
    match is_authenticated_user(&id, &requested_username) {
        AuthenticationStatus::Matching => {
            let connection = pool.get()?;

            let preds = dsl::predictions
                .filter(dsl::username.eq(requested_username))
                .load::<Prediction>(&connection)?;

            let deltas = Delta::belonging_to(&preds)
                .load::<Delta>(&connection)?
                .grouped_by(&preds);

            let full_preds = preds
                .into_iter()
                .zip(deltas)
                .map(|(pred, deltas)| {
                    let dates = DeltaDate::belonging_to(&deltas)
                        .load::<DeltaDate>(&connection)
                        .expect("could not get DeltaDates")
                        .grouped_by(&deltas);

                    let full_delta = deltas
                        .into_iter()
                        .zip(dates)
                        .map(|delta_dates| delta_dates.into())
                        .collect::<Vec<DeltaWithDates>>();

                    (pred, full_delta).into()
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
                let connection = pool.get()?;

                insert_into(dsl::predictions)
                    .values(prediction)
                    .on_conflict_do_nothing()
                    .execute(&connection)?;

                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        _ => Ok(HttpResponse::Unauthorized().finish()),
    }
}
