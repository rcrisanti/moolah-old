use super::*;

mod utils {
    use super::*;

    #[test]
    fn test_month_day_from_i16() {
        assert!(MonthDay::new(1).is_ok());
        assert!(MonthDay::new(31).is_ok());
        assert!(MonthDay::new(0).is_err());
        assert!(MonthDay::new(32).is_err());
    }

    #[test]
    fn test_month_day_into_i16() {
        let day: i16 = MonthDay::new(1).unwrap().into();
        assert_eq!(1, day);

        let day: i16 = MonthDay::new(31).unwrap().into();
        assert_eq!(31, day);
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2022, 1), 31);
        assert_eq!(days_in_month(2022, 2), 28); // not leap year
        assert_eq!(days_in_month(2022, 3), 31);
        assert_eq!(days_in_month(2022, 4), 30);
        assert_eq!(days_in_month(2022, 5), 31);
        assert_eq!(days_in_month(2022, 6), 30);
        assert_eq!(days_in_month(2022, 7), 31);
        assert_eq!(days_in_month(2022, 8), 31);
        assert_eq!(days_in_month(2022, 9), 30);
        assert_eq!(days_in_month(2022, 10), 31);
        assert_eq!(days_in_month(2022, 11), 30);
        assert_eq!(days_in_month(2022, 12), 31);
        assert_eq!(days_in_month(2024, 2), 29); // leap year
    }

    #[test]
    fn test_clipped_date() {
        assert_eq!(
            date_ymd_clipped(2022, 1, MonthDay::new(1).unwrap()),
            NaiveDate::from_ymd(2022, 1, 1)
        );
        assert_eq!(
            date_ymd_clipped(2022, 1, MonthDay::new(31).unwrap()),
            NaiveDate::from_ymd(2022, 1, 31)
        );
        assert_eq!(
            date_ymd_clipped(2022, 2, MonthDay::new(1).unwrap()),
            NaiveDate::from_ymd(2022, 2, 1)
        );
        assert_eq!(
            date_ymd_clipped(2022, 2, MonthDay::new(31).unwrap()),
            NaiveDate::from_ymd(2022, 2, 28)
        );
    }
}

mod monthly_dates {
    use super::*;

    #[test]
    fn test_single_year_matching_simple() {
        assert_eq!(
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2022, 4, 7),
                to: NaiveDate::from_ymd(2022, 9, 7),
                repeat_on_day: MonthDay::new(7).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2022, 4, 7),
                to: NaiveDate::from_ymd(2022, 9, 15),
                repeat_on_day: MonthDay::new(12).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2022, 4, 15),
                to: NaiveDate::from_ymd(2022, 9, 7),
                repeat_on_day: MonthDay::new(12).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2022, 4, 15),
                to: NaiveDate::from_ymd(2022, 9, 7),
                repeat_on_day: MonthDay::new(31).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2020, 1, 15),
                to: NaiveDate::from_ymd(2020, 5, 7),
                repeat_on_day: MonthDay::new(31).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2022, 1, 15),
                to: NaiveDate::from_ymd(2022, 5, 7),
                repeat_on_day: MonthDay::new(31).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2021, 4, 7),
                to: NaiveDate::from_ymd(2022, 3, 7),
                repeat_on_day: MonthDay::new(7).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2021, 4, 7),
                to: NaiveDate::from_ymd(2022, 3, 15),
                repeat_on_day: MonthDay::new(12).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2021, 4, 15),
                to: NaiveDate::from_ymd(2022, 3, 7),
                repeat_on_day: MonthDay::new(12).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2021, 4, 15),
                to: NaiveDate::from_ymd(2022, 4, 7),
                repeat_on_day: MonthDay::new(31).unwrap()
            }
            .dates(),
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
            Repetition::Monthly {
                from: NaiveDate::from_ymd(2019, 4, 15),
                to: NaiveDate::from_ymd(2020, 4, 7),
                repeat_on_day: MonthDay::new(31).unwrap()
            }
            .dates(),
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

mod weekly_dates {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn test_matching_weekdays() {
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 13),
                to: NaiveDate::from_ymd(2022, 1, 17),
                repeat_on_weekday: Weekday::Mon
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 13),
                to: NaiveDate::from_ymd(2022, 1, 18),
                repeat_on_weekday: Weekday::Tue
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 14),
                to: NaiveDate::from_ymd(2022, 1, 17),
                repeat_on_weekday: Weekday::Mon
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 14),
                to: NaiveDate::from_ymd(2022, 1, 17),
                repeat_on_weekday: Weekday::Tue
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 14),
                to: NaiveDate::from_ymd(2022, 1, 19),
                repeat_on_weekday: Weekday::Tue
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 14),
                to: NaiveDate::from_ymd(2022, 1, 19),
                repeat_on_weekday: Weekday::Thu
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 15),
                to: NaiveDate::from_ymd(2022, 1, 20),
                repeat_on_weekday: Weekday::Tue
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 14),
                to: NaiveDate::from_ymd(2022, 1, 23),
                repeat_on_weekday: Weekday::Sat
            }
            .dates(),
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
        assert_eq!(
            Repetition::Weekly {
                from: NaiveDate::from_ymd(2021, 12, 17),
                to: NaiveDate::from_ymd(2022, 1, 18),
                repeat_on_weekday: Weekday::Thu
            }
            .dates(),
            vec![
                NaiveDate::from_ymd(2021, 12, 23),
                NaiveDate::from_ymd(2021, 12, 30),
                NaiveDate::from_ymd(2022, 1, 6),
                NaiveDate::from_ymd(2022, 1, 13),
            ]
        )
    }
}

mod daily_dates {
    use super::*;

    #[test]
    fn test_dates() {
        let dates = Repetition::Daily {
            from: NaiveDate::from_ymd(2021, 12, 13),
            to: NaiveDate::from_ymd(2022, 1, 5),
        }
        .dates();

        assert_eq!(*dates.first().unwrap(), NaiveDate::from_ymd(2021, 12, 13));
        assert_eq!(*dates.last().unwrap(), NaiveDate::from_ymd(2022, 1, 5));
        assert_eq!(dates.len(), 24);
    }
}

mod once_dates {
    use super::*;

    #[test]
    fn test_date() {
        let dates = Repetition::Once {
            on: NaiveDate::from_ymd(2022, 1, 27),
        }
        .dates();

        assert_eq!(dates.len(), 1);
        assert_eq!(*dates.first().unwrap(), NaiveDate::from_ymd(2022, 1, 27));
    }
}
