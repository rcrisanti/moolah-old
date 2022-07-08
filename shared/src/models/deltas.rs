pub mod daily_deltas;
pub mod monthly_deltas;
pub mod once_deltas;
pub mod weekly_deltas;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub use daily_deltas::{DailyDelta, NewDailyDelta};
pub use monthly_deltas::{MonthlyDelta, NewMonthlyDelta};
pub use once_deltas::{NewOnceDelta, OnceDelta};
pub use weekly_deltas::{NewWeeklyDelta, WeeklyDelta};

pub trait DateSequence {
    fn dates(&self) -> Vec<chrono::NaiveDate>;
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub enum DateRepetition {
    Monthly(i16), // day of month repeated on
    Weekly(i16),  // day of week 1=Mon, 7=Sun
    Daily,
    Once,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Delta {
    pub id: i32,
    pub prediction_id: i32,
    pub name: String,
    pub value: f32,
    pub positive_uncertainty: f32,
    pub negative_uncertainty: f32,
    pub dates: Vec<NaiveDate>,
    pub repetition: DateRepetition,
}
