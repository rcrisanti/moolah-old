use diesel::{
    backend::Backend,
    sql_types::SmallInt,
    types::{FromSql, ToSql},
};
use serde::{Deserialize, Serialize};

use super::super::Repetition;
use crate::MoolahSharedError;

#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = SmallInt)]
pub enum DbDateRepetition {
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Once = 4,
}

impl TryFrom<i16> for DbDateRepetition {
    type Error = MoolahSharedError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DbDateRepetition::Monthly),
            2 => Ok(DbDateRepetition::Weekly),
            3 => Ok(DbDateRepetition::Daily),
            4 => Ok(DbDateRepetition::Once),
            _ => Err(MoolahSharedError::RepetitionError(
                "unrecognized repetition variant",
            )),
        }
    }
}

impl From<Repetition> for DbDateRepetition {
    fn from(rep: Repetition) -> Self {
        match rep {
            Repetition::Monthly {
                from: _,
                to: _,
                repeat_on_day: _,
            } => Self::Monthly,
            Repetition::Weekly {
                from: _,
                to: _,
                repeat_on_weekday: _,
            } => Self::Weekly,
            Repetition::Daily { from: _, to: _ } => Self::Daily,
            Repetition::Once { on: _ } => Self::Once,
        }
    }
}

impl<DB> ToSql<SmallInt, DB> for DbDateRepetition
where
    DB: Backend,
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        (*self as i16).to_sql(out)
    }
}

impl<DB> FromSql<SmallInt, DB> for DbDateRepetition
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        match i16::from_sql(bytes)? {
            1 => Ok(Self::Monthly),
            2 => Ok(Self::Weekly),
            3 => Ok(Self::Daily),
            4 => Ok(Self::Once),
            x => Err(format!("unrecognized variant {}", x).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};

    use super::*;
    use crate::models::deltas::app::repetition::MonthDay;

    #[test]
    fn test_int_to_db_date_repetition() {
        assert_eq!(DbDateRepetition::Monthly, 1.try_into().unwrap());
        assert_eq!(DbDateRepetition::Weekly, 2.try_into().unwrap());
        assert_eq!(DbDateRepetition::Daily, 3.try_into().unwrap());
        assert_eq!(DbDateRepetition::Once, 4.try_into().unwrap());
    }

    #[test]
    fn test_db_date_repetition_from_repetition() {
        let now = Local::now().date().naive_utc();

        assert_eq!(
            DbDateRepetition::Monthly,
            Repetition::Monthly {
                from: now,
                to: now,
                repeat_on_day: MonthDay::new(1).unwrap()
            }
            .into()
        );

        assert_eq!(
            DbDateRepetition::Weekly,
            Repetition::Weekly {
                from: now,
                to: now,
                repeat_on_weekday: now.weekday()
            }
            .into()
        );

        assert_eq!(
            DbDateRepetition::Daily,
            Repetition::Daily { from: now, to: now }.into()
        );

        assert_eq!(DbDateRepetition::Once, Repetition::Once { on: now }.into());
    }
}
