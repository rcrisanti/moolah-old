use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::{insert_into, prelude::*};
use shared::models::{
    DailyDelta, Delta, MonthlyDelta, NewPrediction, OnceDelta, Prediction, PredictionWithDeltas,
    WeeklyDelta,
};
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

            let monthly_deltas = MonthlyDelta::belonging_to(&preds)
                .load::<MonthlyDelta>(&connection)?
                .grouped_by(&preds);

            let weekly_deltas = WeeklyDelta::belonging_to(&preds)
                .load::<WeeklyDelta>(&connection)?
                .grouped_by(&preds);

            let daily_deltas = DailyDelta::belonging_to(&preds)
                .load::<DailyDelta>(&connection)?
                .grouped_by(&preds);

            let once_deltas = OnceDelta::belonging_to(&preds)
                .load::<OnceDelta>(&connection)?
                .grouped_by(&preds);

            let full_preds = preds
                .into_iter()
                .zip(monthly_deltas)
                .zip(weekly_deltas)
                .zip(daily_deltas)
                .zip(once_deltas)
                .map(|((((pred, monthly), weekly), daily), once)| {
                    let deltas = monthly
                        .into_iter()
                        .map(|d| d.into())
                        .chain(weekly.into_iter().map(|d| d.into()))
                        .chain(daily.into_iter().map(|d| d.into()))
                        .chain(once.into_iter().map(|d| d.into()))
                        .collect::<Vec<Delta>>();
                    (pred, deltas).into()
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
