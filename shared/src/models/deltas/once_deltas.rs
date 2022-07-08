use serde::{Deserialize, Serialize};

use super::{DateRepetition, DateSequence, Delta};
use crate::models::Prediction;
use crate::schema::once_deltas;

#[derive(Queryable, Identifiable, Serialize, Associations, Deserialize, Clone, PartialEq)]
#[belongs_to(Prediction)]
pub struct OnceDelta {
    pub id: i32,
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
}

impl DateSequence for OnceDelta {
    fn dates(&self) -> Vec<chrono::NaiveDate> {
        vec![self.start_on]
    }
}

impl Into<Delta> for OnceDelta {
    fn into(self) -> Delta {
        Delta {
            id: self.id,
            prediction_id: self.prediction_id,
            name: self.name.clone(),
            value: self.value,
            positive_uncertainty: self.positive_uncertainty,
            negative_uncertainty: self.negative_uncertainty,
            dates: self.dates(),
            repetition: DateRepetition::Once,
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "once_deltas"]
pub struct NewOnceDelta {
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
}
