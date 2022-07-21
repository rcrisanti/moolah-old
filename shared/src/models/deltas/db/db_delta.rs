use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::db_date_repetition::DbDateRepetition;
use crate::models::{NewDelta, Prediction, Repetition};
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

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "deltas"]
pub struct NewDbDelta {
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

impl NewDbDelta {
    pub fn prediction_id(&self) -> i32 {
        self.prediction_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn positive_uncertainty(&self) -> f32 {
        self.positive_uncertainty
    }

    pub fn negative_uncertainty(&self) -> f32 {
        self.negative_uncertainty
    }
}

impl From<NewDelta> for NewDbDelta {
    fn from(new_delta: NewDelta) -> Self {
        match new_delta.repetition() {
            Repetition::Monthly {
                from,
                to,
                repeat_on_day,
            } => NewDbDelta {
                prediction_id: new_delta.prediction_id(),
                name: new_delta.name().to_string(),
                value: new_delta.value(),
                positive_uncertainty: new_delta.positive_uncertainty(),
                negative_uncertainty: new_delta.negative_uncertainty(),
                repetition: new_delta.repetition().into(),
                start_on: *from,
                end_on: Some(*to),
                repeat_day: Some(repeat_on_day.into()),
                repeat_weekday: None,
            },
            Repetition::Weekly {
                from,
                to,
                repeat_on_weekday,
            } => NewDbDelta {
                prediction_id: new_delta.prediction_id(),
                name: new_delta.name().to_string(),
                value: new_delta.value(),
                positive_uncertainty: new_delta.positive_uncertainty(),
                negative_uncertainty: new_delta.negative_uncertainty(),
                repetition: new_delta.repetition().into(),
                start_on: *from,
                end_on: Some(*to),
                repeat_day: None,
                repeat_weekday: Some(repeat_on_weekday.to_string()),
            },
            Repetition::Daily { from, to } => NewDbDelta {
                prediction_id: new_delta.prediction_id(),
                name: new_delta.name().to_string(),
                value: new_delta.value(),
                positive_uncertainty: new_delta.positive_uncertainty(),
                negative_uncertainty: new_delta.negative_uncertainty(),
                repetition: new_delta.repetition().into(),
                start_on: *from,
                end_on: Some(*to),
                repeat_day: None,
                repeat_weekday: None,
            },
            Repetition::Once { on } => NewDbDelta {
                prediction_id: new_delta.prediction_id(),
                name: new_delta.name().to_string(),
                value: new_delta.value(),
                positive_uncertainty: new_delta.positive_uncertainty(),
                negative_uncertainty: new_delta.negative_uncertainty(),
                repetition: new_delta.repetition().into(),
                start_on: *on,
                end_on: None,
                repeat_day: None,
                repeat_weekday: None,
            },
        }
    }
}
