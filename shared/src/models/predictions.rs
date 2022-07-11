use serde::{Deserialize, Serialize};

use super::Delta;
use crate::schema::predictions;

#[derive(Queryable, Identifiable, Serialize)]
pub struct Prediction {
    id: i32,
    username: String,
    name: String,
}

impl Prediction {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PredictionWithDeltas {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub deltas: Vec<Delta>,
}

impl From<(Prediction, Vec<Delta>)> for PredictionWithDeltas {
    fn from((pred, deltas): (Prediction, Vec<Delta>)) -> Self {
        PredictionWithDeltas {
            id: pred.id,
            username: pred.username,
            name: pred.name,
            deltas,
        }
    }
}
