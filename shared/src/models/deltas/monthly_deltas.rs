use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use super::{DateRepetition, DateSequence, Delta};
use crate::models::Prediction;
use crate::schema::monthly_deltas;

#[derive(Queryable, Identifiable, Serialize, Associations, Deserialize, Clone, PartialEq)]
#[belongs_to(Prediction)]
pub struct MonthlyDelta {
    pub id: i32,
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
    pub end_on: chrono::NaiveDate,
    pub repeat_day: i16,
}

fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
    .try_into()
    .expect("should never panic, but means # of days in month cannot be cast to u32")
}

fn date_ymd_clipped(year: i32, month: u32, day: u32) -> NaiveDate {
    let date = NaiveDate::from_ymd_opt(year, month, day);

    match date {
        Some(date) => date,
        None => NaiveDate::from_ymd(year, month, days_in_month(year, month)),
    }
}

impl DateSequence for MonthlyDelta {
    fn dates(&self) -> Vec<chrono::NaiveDate> {
        let repeat_day = self
            .repeat_day
            .try_into()
            .expect("repeat day could not be cast to u32");

        let start_on = date_ymd_clipped(self.start_on.year(), self.start_on.month(), repeat_day);

        let mut date = if start_on >= self.start_on {
            start_on
        } else {
            // need to start next month
            date_ymd_clipped(self.start_on.year(), self.start_on.month() + 1, repeat_day)
        };

        let mut dates: Vec<NaiveDate> = Vec::new();
        while date <= self.end_on {
            dates.push(date);
            date = date_ymd_clipped(
                match date.month() {
                    12 => date.year() + 1,
                    _ => date.year(),
                },
                match date.month() {
                    12 => 1,
                    _ => date.month() + 1,
                },
                repeat_day,
            );
        }
        dates
    }
}

impl Into<Delta> for MonthlyDelta {
    fn into(self) -> Delta {
        Delta {
            id: self.id,
            prediction_id: self.prediction_id,
            name: self.name.clone(),
            value: self.value,
            positive_uncertainty: self.positive_uncertainty,
            negative_uncertainty: self.negative_uncertainty,
            dates: self.dates(),
            repetition: DateRepetition::Monthly(self.repeat_day),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "monthly_deltas"]
pub struct NewMonthlyDelta {
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub start_on: chrono::NaiveDate,
    pub end_on: chrono::NaiveDate,
    pub repeat_day: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dates(from: NaiveDate, to: NaiveDate, repeat_day: i16) -> Vec<NaiveDate> {
        MonthlyDelta {
            id: 1,
            prediction_id: 1,
            name: "test".to_string(),
            value: 0.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            start_on: from,
            end_on: to,
            repeat_day,
        }
        .dates()
    }

    #[test]
    fn test_single_year_matching_simple() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2022, 4, 7),
                NaiveDate::from_ymd(2022, 9, 7),
                7
            ),
            vec![
                NaiveDate::from_ymd(2022, 4, 7),
                NaiveDate::from_ymd(2022, 5, 7),
                NaiveDate::from_ymd(2022, 6, 7),
                NaiveDate::from_ymd(2022, 7, 7),
                NaiveDate::from_ymd(2022, 8, 7),
                NaiveDate::from_ymd(2022, 9, 7)
            ]
        )
    }

    #[test]
    fn test_single_year_nonmatching_early_late() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2022, 4, 7),
                NaiveDate::from_ymd(2022, 9, 15),
                12
            ),
            vec![
                NaiveDate::from_ymd(2022, 4, 12),
                NaiveDate::from_ymd(2022, 5, 12),
                NaiveDate::from_ymd(2022, 6, 12),
                NaiveDate::from_ymd(2022, 7, 12),
                NaiveDate::from_ymd(2022, 8, 12),
                NaiveDate::from_ymd(2022, 9, 12)
            ]
        )
    }

    #[test]
    fn test_single_year_nonmatching_late_early() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2022, 4, 15),
                NaiveDate::from_ymd(2022, 9, 7),
                12
            ),
            vec![
                NaiveDate::from_ymd(2022, 5, 12),
                NaiveDate::from_ymd(2022, 6, 12),
                NaiveDate::from_ymd(2022, 7, 12),
                NaiveDate::from_ymd(2022, 8, 12),
            ]
        )
    }

    #[test]
    fn test_single_year_nonmatching_end_of_month() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2022, 4, 15),
                NaiveDate::from_ymd(2022, 9, 7),
                31
            ),
            vec![
                NaiveDate::from_ymd(2022, 4, 30),
                NaiveDate::from_ymd(2022, 5, 31),
                NaiveDate::from_ymd(2022, 6, 30),
                NaiveDate::from_ymd(2022, 7, 31),
                NaiveDate::from_ymd(2022, 8, 31),
            ]
        )
    }

    #[test]
    fn test_single_year_nonmatching_end_of_month_leap_year() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2020, 1, 15),
                NaiveDate::from_ymd(2020, 5, 7),
                31
            ),
            vec![
                NaiveDate::from_ymd(2020, 1, 31),
                NaiveDate::from_ymd(2020, 2, 29),
                NaiveDate::from_ymd(2020, 3, 31),
                NaiveDate::from_ymd(2020, 4, 30),
            ]
        )
    }

    #[test]
    fn test_single_year_nonmatching_end_of_month_non_leap_year() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2022, 1, 15),
                NaiveDate::from_ymd(2022, 5, 7),
                31
            ),
            vec![
                NaiveDate::from_ymd(2022, 1, 31),
                NaiveDate::from_ymd(2022, 2, 28),
                NaiveDate::from_ymd(2022, 3, 31),
                NaiveDate::from_ymd(2022, 4, 30),
            ]
        )
    }

    #[test]
    fn test_multi_year_matching_simple() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2021, 4, 7),
                NaiveDate::from_ymd(2022, 3, 7),
                7
            ),
            vec![
                NaiveDate::from_ymd(2021, 4, 7),
                NaiveDate::from_ymd(2021, 5, 7),
                NaiveDate::from_ymd(2021, 6, 7),
                NaiveDate::from_ymd(2021, 7, 7),
                NaiveDate::from_ymd(2021, 8, 7),
                NaiveDate::from_ymd(2021, 9, 7),
                NaiveDate::from_ymd(2021, 10, 7),
                NaiveDate::from_ymd(2021, 11, 7),
                NaiveDate::from_ymd(2021, 12, 7),
                NaiveDate::from_ymd(2022, 1, 7),
                NaiveDate::from_ymd(2022, 2, 7),
                NaiveDate::from_ymd(2022, 3, 7)
            ]
        )
    }

    #[test]
    fn test_multi_year_nonmatching_early_late() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2021, 4, 7),
                NaiveDate::from_ymd(2022, 3, 15),
                12
            ),
            vec![
                NaiveDate::from_ymd(2021, 4, 12),
                NaiveDate::from_ymd(2021, 5, 12),
                NaiveDate::from_ymd(2021, 6, 12),
                NaiveDate::from_ymd(2021, 7, 12),
                NaiveDate::from_ymd(2021, 8, 12),
                NaiveDate::from_ymd(2021, 9, 12),
                NaiveDate::from_ymd(2021, 10, 12),
                NaiveDate::from_ymd(2021, 11, 12),
                NaiveDate::from_ymd(2021, 12, 12),
                NaiveDate::from_ymd(2022, 1, 12),
                NaiveDate::from_ymd(2022, 2, 12),
                NaiveDate::from_ymd(2022, 3, 12)
            ]
        )
    }

    #[test]
    fn test_multi_year_nonmatching_late_early() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2021, 4, 15),
                NaiveDate::from_ymd(2022, 3, 7),
                12
            ),
            vec![
                NaiveDate::from_ymd(2021, 5, 12),
                NaiveDate::from_ymd(2021, 6, 12),
                NaiveDate::from_ymd(2021, 7, 12),
                NaiveDate::from_ymd(2021, 8, 12),
                NaiveDate::from_ymd(2021, 9, 12),
                NaiveDate::from_ymd(2021, 10, 12),
                NaiveDate::from_ymd(2021, 11, 12),
                NaiveDate::from_ymd(2021, 12, 12),
                NaiveDate::from_ymd(2022, 1, 12),
                NaiveDate::from_ymd(2022, 2, 12),
            ]
        )
    }

    #[test]
    fn test_multi_year_nonmatching_end_of_month_non_leap_year() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2021, 4, 15),
                NaiveDate::from_ymd(2022, 4, 7),
                31
            ),
            vec![
                NaiveDate::from_ymd(2021, 4, 30),
                NaiveDate::from_ymd(2021, 5, 31),
                NaiveDate::from_ymd(2021, 6, 30),
                NaiveDate::from_ymd(2021, 7, 31),
                NaiveDate::from_ymd(2021, 8, 31),
                NaiveDate::from_ymd(2021, 9, 30),
                NaiveDate::from_ymd(2021, 10, 31),
                NaiveDate::from_ymd(2021, 11, 30),
                NaiveDate::from_ymd(2021, 12, 31),
                NaiveDate::from_ymd(2022, 1, 31),
                NaiveDate::from_ymd(2022, 2, 28),
                NaiveDate::from_ymd(2022, 3, 31),
            ]
        )
    }

    #[test]
    fn test_multi_year_nonmatching_end_of_month_leap_year() {
        assert_eq!(
            dates(
                NaiveDate::from_ymd(2019, 4, 15),
                NaiveDate::from_ymd(2020, 4, 7),
                31
            ),
            vec![
                NaiveDate::from_ymd(2019, 4, 30),
                NaiveDate::from_ymd(2019, 5, 31),
                NaiveDate::from_ymd(2019, 6, 30),
                NaiveDate::from_ymd(2019, 7, 31),
                NaiveDate::from_ymd(2019, 8, 31),
                NaiveDate::from_ymd(2019, 9, 30),
                NaiveDate::from_ymd(2019, 10, 31),
                NaiveDate::from_ymd(2019, 11, 30),
                NaiveDate::from_ymd(2019, 12, 31),
                NaiveDate::from_ymd(2020, 1, 31),
                NaiveDate::from_ymd(2020, 2, 29),
                NaiveDate::from_ymd(2020, 3, 31),
            ]
        )
    }
}
