use serde::{Deserialize, Serialize};

use super::{DeltaDate, Prediction};
use crate::schema::deltas;

#[derive(Queryable, Identifiable, Serialize, Associations, Deserialize, Clone, PartialEq)]
#[belongs_to(Prediction)]
pub struct Delta {
    pub id: i32,
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "deltas"]
pub struct NewDelta {
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
}

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub struct DeltaWithDates {
    pub id: i32,
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub dates: Vec<DeltaDate>,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
}

impl From<(Delta, Vec<DeltaDate>)> for DeltaWithDates {
    fn from((delta, dates): (Delta, Vec<DeltaDate>)) -> Self {
        DeltaWithDates {
            id: delta.id,
            prediction_id: delta.prediction_id,
            name: delta.name,
            value: delta.value,
            dates,
            positive_uncertainty: delta.positive_uncertainty,
            negative_uncertainty: delta.negative_uncertainty,
        }
    }
}
