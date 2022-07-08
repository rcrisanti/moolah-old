use serde::{Deserialize, Serialize};

use super::{DateRepetition, DateSequence, Delta};
use crate::models::Prediction;
use crate::schema::daily_deltas;

#[derive(Queryable, Identifiable, Serialize, Associations, Deserialize, Clone, PartialEq)]
#[belongs_to(Prediction)]
pub struct DailyDelta {
    pub id: i32,
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
    pub end_on: chrono::NaiveDate,
}

impl DateSequence for DailyDelta {
    fn dates(&self) -> Vec<chrono::NaiveDate> {
        let days = (self.end_on - self.start_on).num_days() + 1;
        self.start_on
            .iter_days()
            .take(days.try_into().expect("could not convert days to usize"))
            .collect()
    }
}

impl Into<Delta> for DailyDelta {
    fn into(self) -> Delta {
        Delta {
            id: self.id,
            prediction_id: self.prediction_id,
            name: self.name.clone(),
            value: self.value,
            positive_uncertainty: self.positive_uncertainty,
            negative_uncertainty: self.negative_uncertainty,
            dates: self.dates(),
            repetition: DateRepetition::Daily,
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "daily_deltas"]
pub struct NewDailyDelta {
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
    pub end_on: chrono::NaiveDate,
}

#[cfg(test)]
mod test {
    use super::{DailyDelta, DateSequence};
    use chrono::NaiveDate;

    #[test]
    fn test_dates() {
        let delta = DailyDelta {
            id: 1,
            prediction_id: 1,
            name: "test".to_string(),
            value: 123.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 13),
            end_on: NaiveDate::from_ymd(2022, 1, 5),
        };

        let dates = delta.dates();

        assert_eq!(*dates.first().unwrap(), NaiveDate::from_ymd(2021, 12, 13));
        assert_eq!(*dates.last().unwrap(), NaiveDate::from_ymd(2022, 1, 5));
        assert_eq!(dates.len(), 24);
    }
}
