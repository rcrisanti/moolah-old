use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::db_date_repetition::DbDateRepetition;
use crate::models::{Delta, Prediction, Repetition};
use crate::schema::deltas;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[table_name = "deltas"]
#[belongs_to(Prediction)]
pub struct DbDelta {
    pub(crate) id: i32,
    pub(crate) prediction_id: i32,
    pub(crate) name: String,
    pub(crate) value: f32,
    pub(crate) positive_uncertainty: f32,
    pub(crate) negative_uncertainty: f32,
    pub(crate) repetition: DbDateRepetition,
    pub(crate) start_on: NaiveDate,
    pub(crate) end_on: Option<NaiveDate>,
    pub(crate) repeat_day: Option<i16>,
    pub(crate) repeat_weekday: Option<String>,
}

impl DbDelta {
    pub fn new(
        id: i32,
        prediction_id: i32,
        name: String,
        value: f32,
        positive_uncertainty: f32,
        negative_uncertainty: f32,
        repetition: DbDateRepetition,
        start_on: NaiveDate,
        end_on: Option<NaiveDate>,
        repeat_day: Option<i16>,
        repeat_weekday: Option<String>,
    ) -> Self {
        DbDelta {
            id,
            prediction_id,
            name,
            value,
            positive_uncertainty,
            negative_uncertainty,
            repetition,
            start_on,
            end_on,
            repeat_day,
            repeat_weekday,
        }
    }
}

impl<'a> From<&'a Delta> for DbDelta {
    fn from(delta: &'a Delta) -> Self {
        match &delta.repetition() {
            Repetition::Monthly {
                from,
                to,
                repeat_on_day,
            } => DbDelta {
                id: delta.id(),
                prediction_id: delta.prediction_id(),
                name: delta.name().to_string(),
                value: delta.value(),
                positive_uncertainty: delta.positive_uncertainty(),
                negative_uncertainty: delta.negative_uncertainty(),
                repetition: DbDateRepetition::Monthly,
                start_on: *from,
                end_on: Some(*to),
                repeat_day: Some((*repeat_on_day).into()),
                repeat_weekday: None,
            },
            Repetition::Weekly {
                from,
                to,
                repeat_on_weekday,
            } => DbDelta {
                id: delta.id(),
                prediction_id: delta.prediction_id(),
                name: delta.name().to_string(),
                value: delta.value(),
                positive_uncertainty: delta.positive_uncertainty(),
                negative_uncertainty: delta.negative_uncertainty(),
                repetition: DbDateRepetition::Weekly,
                start_on: *from,
                end_on: Some(*to),
                repeat_day: None,
                repeat_weekday: Some((*repeat_on_weekday).to_string()),
            },
            Repetition::Daily { from, to } => DbDelta {
                id: delta.id(),
                prediction_id: delta.prediction_id(),
                name: delta.name().to_string(),
                value: delta.value(),
                positive_uncertainty: delta.positive_uncertainty(),
                negative_uncertainty: delta.negative_uncertainty(),
                repetition: DbDateRepetition::Monthly,
                start_on: *from,
                end_on: Some(*to),
                repeat_day: None,
                repeat_weekday: None,
            },
            Repetition::Once { on } => DbDelta {
                id: delta.id(),
                prediction_id: delta.prediction_id(),
                name: delta.name().to_string(),
                value: delta.value(),
                positive_uncertainty: delta.positive_uncertainty(),
                negative_uncertainty: delta.negative_uncertainty(),
                repetition: DbDateRepetition::Monthly,
                start_on: *on,
                end_on: None,
                repeat_day: None,
                repeat_weekday: None,
            },
        }
    }
}
