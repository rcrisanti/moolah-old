use serde::{Deserialize, Serialize};

use crate::schema::predictions;

use super::DeltaWithDates;

#[derive(Queryable, Identifiable, Serialize)]
pub struct Prediction {
    pub id: i32,
    pub username: String,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "predictions"]
pub struct NewPrediction {
    pub username: String,
    pub name: String,
}

impl NewPrediction {
    pub fn new(username: String, name: String) -> Self {
        NewPrediction {
            username: username.to_lowercase(),
            name,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PredictionWithDeltas {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub deltas: Vec<DeltaWithDates>,
}

impl From<(Prediction, Vec<DeltaWithDates>)> for PredictionWithDeltas {
    fn from((pred, deltas): (Prediction, Vec<DeltaWithDates>)) -> Self {
        PredictionWithDeltas {
            id: pred.id,
            username: pred.username,
            name: pred.name,
            deltas,
        }
    }
}
