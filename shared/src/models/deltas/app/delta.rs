#[cfg(test)]
mod tests;

use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::Repetition;
use crate::models::deltas::db::DbDateRepetition;
use crate::{models::DbDelta, MoolahSharedError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    id: i32,
    prediction_id: i32,
    name: String,
    value: f32,
    positive_uncertainty: f32,
    negative_uncertainty: f32,
    repetition: Repetition,
    dates: Vec<NaiveDate>,
}

impl Delta {
    pub fn new(
        id: i32,
        prediction_id: i32,
        name: String,
        value: f32,
        positive_uncertainty: f32,
        negative_uncertainty: f32,
        repetition: Repetition,
    ) -> Delta {
        Delta {
            id,
            prediction_id,
            name,
            value,
            positive_uncertainty,
            negative_uncertainty,
            repetition,
            dates: repetition.dates(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

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

    pub fn repetition(&self) -> Repetition {
        self.repetition
    }

    pub fn dates(&self) -> &Vec<NaiveDate> {
        &self.dates
    }
}

impl TryFrom<DbDelta> for Delta {
    type Error = MoolahSharedError;

    fn try_from(value: DbDelta) -> Result<Self, Self::Error> {
        let repetition = match value.repetition {
            DbDateRepetition::Monthly => Repetition::Monthly {
                from: value.start_on,
                to: value.end_on.ok_or_else(|| {
                    MoolahSharedError::DeltaConversionError(
                        "monthly repetition does not have end date",
                    )
                })?,
                repeat_on_day: value
                    .repeat_day
                    .ok_or_else(|| {
                        MoolahSharedError::DeltaConversionError(
                            "monthly repetition does not have repeat day",
                        )
                    })?
                    .try_into()
                    .or_else(|_| {
                        Err(MoolahSharedError::DeltaConversionError(
                            "monthly repetition repeat day is not in range [1, 31]",
                        ))
                    })?,
            },
            DbDateRepetition::Weekly => Repetition::Weekly {
                from: value.start_on,
                to: value.end_on.ok_or_else(|| {
                    MoolahSharedError::DeltaConversionError(
                        "weekly repetition does not have end date",
                    )
                })?,
                repeat_on_weekday: chrono::Weekday::from_str(&value.repeat_weekday.ok_or_else(
                    || {
                        MoolahSharedError::DeltaConversionError(
                            "weekly repetition does not have repeat weekday",
                        )
                    },
                )?)
                .or_else(|_| {
                    Err(MoolahSharedError::DeltaConversionError(
                        "could not convert to weekday",
                    ))
                })?,
            },
            DbDateRepetition::Daily => Repetition::Daily {
                from: value.start_on,
                to: value.end_on.ok_or_else(|| {
                    MoolahSharedError::DeltaConversionError(
                        "daily repetition does not have end date",
                    )
                })?,
            },
            DbDateRepetition::Once => Repetition::Once { on: value.start_on },
        };

        Ok(Delta::new(
            value.id,
            value.prediction_id,
            value.name,
            value.value,
            value.positive_uncertainty,
            value.negative_uncertainty,
            repetition,
        ))
    }
}
