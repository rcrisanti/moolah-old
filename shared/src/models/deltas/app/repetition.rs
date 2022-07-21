#[cfg(test)]
mod tests;

use std::fmt::Display;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::MoolahSharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Repetition {
    Monthly {
        from: NaiveDate,
        to: NaiveDate,
        repeat_on_day: MonthDay,
    },
    Weekly {
        from: NaiveDate,
        to: NaiveDate,
        repeat_on_weekday: chrono::Weekday,
    },
    Daily {
        from: NaiveDate,
        to: NaiveDate,
    },
    Once {
        on: NaiveDate,
    },
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

fn date_ymd_clipped(year: i32, month: u32, day: MonthDay) -> NaiveDate {
    let date = NaiveDate::from_ymd_opt(year, month, day.into());

    match date {
        Some(date) => date,
        None => NaiveDate::from_ymd(year, month, days_in_month(year, month)),
    }
}

impl Repetition {
    pub fn dates(&self) -> Vec<NaiveDate> {
        match self {
            Repetition::Monthly {
                from: start,
                to: end,
                repeat_on_day,
            } => {
                let start_on = date_ymd_clipped(start.year(), start.month(), *repeat_on_day);

                let mut date = if start_on >= *start {
                    start_on
                } else {
                    // need to start next month
                    date_ymd_clipped(start.year(), start.month() + 1, *repeat_on_day)
                };

                let mut dates: Vec<NaiveDate> = Vec::new();
                while date <= *end {
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
                        *repeat_on_day,
                    );
                }
                dates
            }
            Repetition::Weekly {
                from: start,
                to: end,
                repeat_on_weekday: weekday,
            } => {
                let mut start_offset_days: i64 = ((weekday.number_from_monday() as i32)
                    - (start.weekday().number_from_monday() as i32))
                    .into();
                if start_offset_days < 0 {
                    start_offset_days += 7;
                }

                let start = *start + chrono::Duration::days(start_offset_days);

                let mut end_offset_days: i64 = ((end.weekday().number_from_monday() as i32)
                    - (weekday.number_from_monday() as i32))
                    .into();
                if end_offset_days < 0 {
                    end_offset_days += 7;
                }

                let end = *end - chrono::Duration::days(end_offset_days);

                let weeks = (end - start).num_weeks() + 1;
                start
                    .iter_weeks()
                    .take(weeks.try_into().expect("could not convert weeks to usize"))
                    .collect()
            }
            Repetition::Daily { from, to } => {
                let days = (*to - *from).num_days() + 1;
                from.iter_days()
                    .take(days.try_into().expect("could not convert days to usize"))
                    .collect()
            }
            Repetition::Once { on } => vec![*on],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub struct MonthDay(u8);

impl MonthDay {
    pub fn new(rep: i16) -> Result<Self, MoolahSharedError> {
        if rep > 31 {
            Err(MoolahSharedError::RepetitionError(
                "month day greater than 31".into(),
            ))
        } else if rep < 1 {
            Err(MoolahSharedError::RepetitionError(
                "month day less than 1".into(),
            ))
        } else {
            Ok(MonthDay(
                rep.try_into()
                    .expect("unreachable - already checked MonthDay 1-31"),
            ))
        }
    }
}

impl Display for MonthDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.0,
            match (self.0 / 10, self.0 % 10) {
                (1, _) => "th",
                (_, 1) => "st",
                (_, 2) => "nd",
                (_, 3) => "rd",
                _ => "th",
            }
        )
    }
}

impl PartialOrd for MonthDay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl TryFrom<i16> for MonthDay {
    type Error = MoolahSharedError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        MonthDay::new(value)
    }
}

impl Into<i16> for MonthDay {
    fn into(self) -> i16 {
        self.0.into()
    }
}

impl Into<i16> for &MonthDay {
    fn into(self) -> i16 {
        self.0.into()
    }
}

impl Into<u32> for MonthDay {
    fn into(self) -> u32 {
        self.0.into()
    }
}
