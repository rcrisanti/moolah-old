use chrono::{NaiveDate, Weekday};

use super::*;
use crate::models::deltas::app::repetition::MonthDay;

mod monthly {
    use super::*;

    #[test]
    fn test_proper() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Monthly,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            Some(12),
            None,
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2022, 1, 1),
                to: NaiveDate::from_ymd(2022, 4, 23),
                repeat_on_day: MonthDay::new(12).unwrap(),
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }

    #[test]
    fn test_extra_fields() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Monthly,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            Some(12),
            Some(Weekday::Mon.to_string()),
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2022, 1, 1),
                to: NaiveDate::from_ymd(2022, 4, 23),
                repeat_on_day: MonthDay::new(12).unwrap(),
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }

    #[test]
    fn test_missing_repeat_day() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Monthly,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            None,
            None,
        );

        assert!(Delta::try_from(db_delta).is_err());
    }

    #[test]
    fn test_missing_end_day() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Monthly,
            NaiveDate::from_ymd(2022, 1, 1),
            None,
            Some(12),
            None,
        );

        assert!(Delta::try_from(db_delta).is_err());
    }
}

mod weekly {
    use super::*;

    #[test]
    fn test_proper() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Weekly,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            None,
            Some("Mon".into()),
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2022, 1, 1),
                to: NaiveDate::from_ymd(2022, 4, 23),
                repeat_on_weekday: Weekday::Mon,
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }

    #[test]
    fn test_extra_fields() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Weekly,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            Some(12),
            Some("Mon".into()),
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2022, 1, 1),
                to: NaiveDate::from_ymd(2022, 4, 23),
                repeat_on_weekday: Weekday::Mon,
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }

    #[test]
    fn test_missing_end_day() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Weekly,
            NaiveDate::from_ymd(2022, 1, 1),
            None,
            None,
            Some("Mon".into()),
        );

        assert!(Delta::try_from(db_delta).is_err());
    }

    #[test]
    fn test_missing_weekday() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Weekly,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            None,
            None,
        );
        assert!(Delta::try_from(db_delta).is_err());
    }
}

mod daily {
    use super::*;

    #[test]
    fn test_proper() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Daily,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            None,
            None,
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Daily {
                from: NaiveDate::from_ymd(2022, 1, 1),
                to: NaiveDate::from_ymd(2022, 4, 23),
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }

    #[test]
    fn test_extra_fields() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Daily,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            Some(12),
            Some("Fri".into()),
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Daily {
                from: NaiveDate::from_ymd(2022, 1, 1),
                to: NaiveDate::from_ymd(2022, 4, 23),
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }

    #[test]
    fn test_missing_end_day() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Daily,
            NaiveDate::from_ymd(2022, 1, 1),
            None,
            None,
            None,
        );
        assert!(Delta::try_from(db_delta).is_err());
    }
}

mod once {
    use super::*;

    #[test]
    fn test_proper() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Once,
            NaiveDate::from_ymd(2022, 1, 1),
            None,
            None,
            None,
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Once {
                on: NaiveDate::from_ymd(2022, 1, 1),
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }

    #[test]
    fn test_extra_fields() {
        let db_delta = DbDelta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            DbDateRepetition::Once,
            NaiveDate::from_ymd(2022, 1, 1),
            Some(NaiveDate::from_ymd(2022, 4, 23)),
            Some(12),
            Some("Thu".into()),
        );
        let expected = Delta::new(
            1,
            1,
            "test".into(),
            0.,
            0.,
            0.,
            Repetition::Once {
                on: NaiveDate::from_ymd(2022, 1, 1),
            },
        );

        assert_eq!(expected, db_delta.try_into().unwrap());
    }
}
