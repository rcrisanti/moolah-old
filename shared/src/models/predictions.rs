use serde::{Deserialize, Serialize};

use super::Delta;
use crate::schema::predictions;

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize, Clone)]
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

impl From<PredictionWithDeltas> for Prediction {
    fn from(pred: PredictionWithDeltas) -> Self {
        Prediction {
            id: pred.id,
            username: pred.username,
            name: pred.name,
        }
    }
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[table_name = "predictions"]
pub struct NewPrediction {
    username: String,
    name: String,
}

impl NewPrediction {
    pub fn new(username: String, name: String) -> Self {
        NewPrediction {
            username: username.to_lowercase(),
            name: name.to_lowercase(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PredictionWithDeltas {
    id: i32,
    username: String,
    name: String,
    deltas: Vec<Delta>,
}

impl PredictionWithDeltas {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn deltas(&self) -> &[Delta] {
        &self.deltas
    }
}

impl From<Prediction> for PredictionWithDeltas {
    fn from(pred: Prediction) -> Self {
        PredictionWithDeltas {
            id: pred.id,
            username: pred.username,
            name: pred.name,
            deltas: Vec::new(),
        }
    }
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
