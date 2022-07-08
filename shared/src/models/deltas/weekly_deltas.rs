use chrono::Datelike;
use serde::{Deserialize, Serialize};

use super::{DateRepetition, DateSequence, Delta};
use crate::models::Prediction;
use crate::schema::weekly_deltas;

#[derive(Queryable, Identifiable, Serialize, Associations, Deserialize, Clone, PartialEq)]
#[belongs_to(Prediction)]
pub struct WeeklyDelta {
    pub id: i32,
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
    pub end_on: chrono::NaiveDate,
    pub repeat_weekday: i16,
}

impl DateSequence for WeeklyDelta {
    fn dates(&self) -> Vec<chrono::NaiveDate> {
        let mut start_offset_days =
            (self.repeat_weekday as i64) - (self.start_on.weekday().number_from_monday() as i64);
        if start_offset_days < 0 {
            start_offset_days += 7;
        }

        let start = self.start_on + chrono::Duration::days(start_offset_days);

        let mut end_offset_days =
            (self.end_on.weekday().number_from_monday() as i64) - (self.repeat_weekday as i64);
        if end_offset_days < 0 {
            end_offset_days += 7;
        }

        let end = self.end_on - chrono::Duration::days(end_offset_days);

        let weeks = (end - start).num_weeks() + 1;
        start
            .iter_weeks()
            .take(weeks.try_into().expect("could not convert weeks to usize"))
            .collect()
    }
}

impl Into<Delta> for WeeklyDelta {
    fn into(self) -> Delta {
        Delta {
            id: self.id,
            prediction_id: self.prediction_id,
            name: self.name.clone(),
            value: self.value,
            positive_uncertainty: self.positive_uncertainty,
            negative_uncertainty: self.negative_uncertainty,
            dates: self.dates(),
            repetition: DateRepetition::Weekly(self.repeat_weekday),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "weekly_deltas"]
pub struct NewWeeklyDelta {
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
    pub end_on: chrono::NaiveDate,
    pub repeat_weekday: i16,
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_matching_weekdays() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 13),
            end_on: NaiveDate::from_ymd(2022, 1, 17),
            repeat_weekday: 1,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 13),
                NaiveDate::from_ymd(2021, 12, 20),
                NaiveDate::from_ymd(2021, 12, 27),
                NaiveDate::from_ymd(2022, 1, 3),
                NaiveDate::from_ymd(2022, 1, 10),
                NaiveDate::from_ymd(2022, 1, 17)
            ]
        )
    }

    #[test]
    fn test_start_mismatched_early() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 13),
            end_on: NaiveDate::from_ymd(2022, 1, 18),
            repeat_weekday: 2,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 14),
                NaiveDate::from_ymd(2021, 12, 21),
                NaiveDate::from_ymd(2021, 12, 28),
                NaiveDate::from_ymd(2022, 1, 4),
                NaiveDate::from_ymd(2022, 1, 11),
                NaiveDate::from_ymd(2022, 1, 18)
            ]
        )
    }

    #[test]
    fn test_start_mismatched_late() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 14),
            end_on: NaiveDate::from_ymd(2022, 1, 17),
            repeat_weekday: 1,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 20),
                NaiveDate::from_ymd(2021, 12, 27),
                NaiveDate::from_ymd(2022, 1, 3),
                NaiveDate::from_ymd(2022, 1, 10),
                NaiveDate::from_ymd(2022, 1, 17)
            ]
        )
    }

    #[test]
    fn test_end_mismatched_early() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 14),
            end_on: NaiveDate::from_ymd(2022, 1, 17),
            repeat_weekday: 2,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 14),
                NaiveDate::from_ymd(2021, 12, 21),
                NaiveDate::from_ymd(2021, 12, 28),
                NaiveDate::from_ymd(2022, 1, 4),
                NaiveDate::from_ymd(2022, 1, 11),
            ]
        )
    }

    #[test]
    fn test_end_mismatched_late() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 14),
            end_on: NaiveDate::from_ymd(2022, 1, 19),
            repeat_weekday: 2,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 14),
                NaiveDate::from_ymd(2021, 12, 21),
                NaiveDate::from_ymd(2021, 12, 28),
                NaiveDate::from_ymd(2022, 1, 4),
                NaiveDate::from_ymd(2022, 1, 11),
                NaiveDate::from_ymd(2022, 1, 18),
            ]
        )
    }

    #[test]
    fn test_mismatched_early() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 14),
            end_on: NaiveDate::from_ymd(2022, 1, 19),
            repeat_weekday: 4,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 16),
                NaiveDate::from_ymd(2021, 12, 23),
                NaiveDate::from_ymd(2021, 12, 30),
                NaiveDate::from_ymd(2022, 1, 6),
                NaiveDate::from_ymd(2022, 1, 13),
            ]
        )
    }

    #[test]
    fn test_mismatched_late() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 15),
            end_on: NaiveDate::from_ymd(2022, 1, 20),
            repeat_weekday: 2,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 21),
                NaiveDate::from_ymd(2021, 12, 28),
                NaiveDate::from_ymd(2022, 1, 4),
                NaiveDate::from_ymd(2022, 1, 11),
                NaiveDate::from_ymd(2022, 1, 18),
            ]
        )
    }

    #[test]
    fn test_mismatched_early_late() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 14),
            end_on: NaiveDate::from_ymd(2022, 1, 23),
            repeat_weekday: 6,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 18),
                NaiveDate::from_ymd(2021, 12, 25),
                NaiveDate::from_ymd(2022, 1, 1),
                NaiveDate::from_ymd(2022, 1, 8),
                NaiveDate::from_ymd(2022, 1, 15),
                NaiveDate::from_ymd(2022, 1, 22),
            ]
        )
    }

    #[test]
    fn test_mismatched_late_early() {
        let delta = WeeklyDelta {
            id: 1,
            prediction_id: 2,
            name: "test".to_string(),
            value: 1.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: NaiveDate::from_ymd(2021, 12, 17),
            end_on: NaiveDate::from_ymd(2022, 1, 18),
            repeat_weekday: 4,
        };

        assert_eq!(
            delta.dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 23),
                NaiveDate::from_ymd(2021, 12, 30),
                NaiveDate::from_ymd(2022, 1, 6),
                NaiveDate::from_ymd(2022, 1, 13),
            ]
        )
    }
}
