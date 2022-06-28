use chrono::format::{DelayedFormat, StrftimeItems};
use serde::{Deserialize, Serialize};

use super::Delta;
use crate::schema::delta_dates;

#[derive(
    Queryable, Insertable, Identifiable, PartialEq, Clone, Deserialize, Serialize, Associations,
)]
#[primary_key(delta_id, date)]
#[table_name = "delta_dates"]
#[belongs_to(Delta)]
pub struct DeltaDate {
    pub delta_id: i32,
    pub date: chrono::NaiveDate,
}

impl Ord for DeltaDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for DeltaDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Eq for DeltaDate {}

impl DeltaDate {
    pub fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        self.date.format(fmt)
    }
}
