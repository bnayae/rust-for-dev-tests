#[cfg(test)]
mod chrono_tests {
    // use super::*;
    use chrono::offset::LocalResult;
    use chrono::{prelude::*, Duration};

    #[test]
    fn chrono1_test() {
        let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
                                                                       // July 8 is 188th day of the year 2014 (`o` for "ordinal")
        assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
        // July 8 is Tuesday in ISO week 28 of the year 2014.
        assert_eq!(
            dt,
            Utc.isoywd(2014, 28, Weekday::Tue)
                .and_hms_opt(9, 10, 11)
                .unwrap()
        );

        let dt = NaiveDate::from_ymd_opt(2014, 7, 8)
            .unwrap()
            .and_hms_milli_opt(9, 10, 11, 12)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap(); // `2014-07-08T09:10:11.012Z`
        assert_eq!(
            dt,
            NaiveDate::from_ymd_opt(2014, 7, 8)
                .unwrap()
                .and_hms_micro_opt(9, 10, 11, 12_000)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        );
        assert_eq!(
            dt,
            NaiveDate::from_ymd_opt(2014, 7, 8)
                .unwrap()
                .and_hms_nano_opt(9, 10, 11, 12_000_000)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        );

        // dynamic verification
        assert_eq!(
            Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33),
            LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap())
        );
        assert_eq!(
            Utc.with_ymd_and_hms(2014, 7, 8, 80, 15, 33),
            LocalResult::None
        );
        assert_eq!(
            Utc.with_ymd_and_hms(2014, 7, 38, 21, 15, 33),
            LocalResult::None
        );

        // other time zone objects can be used to construct a local datetime.
        // obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
        let local_dt = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2014, 7, 8)
                    .unwrap()
                    .and_hms_milli_opt(9, 10, 11, 12)
                    .unwrap(),
            )
            .unwrap();
        let fixed_dt = FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2014, 7, 8)
                    .unwrap()
                    .and_hms_milli_opt(18, 10, 11, 12)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(dt, local_dt);
        assert_eq!(dt, fixed_dt);
    }

    #[test]
    fn chrono2_test() {
        let dt = FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(2014, 11, 28)
                    .unwrap()
                    .and_hms_nano_opt(21, 45, 59, 324310806)
                    .unwrap(),
            )
            .unwrap();

        // property accessors
        assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
        assert_eq!((dt.month0(), dt.day0()), (10, 27)); // for unfortunate souls
        assert_eq!((dt.hour(), dt.minute(), dt.second()), (21, 45, 59));
        assert_eq!(dt.weekday(), Weekday::Fri);
        assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7
        assert_eq!(dt.ordinal(), 332); // the day of year
        assert_eq!(dt.num_days_from_ce(), 735565); // the number of days from and including Jan 1, 1

        // time zone accessor and manipulation
        assert_eq!(dt.offset().fix().local_minus_utc(), 9 * 3600);
        assert_eq!(dt.timezone(), FixedOffset::east_opt(9 * 3600).unwrap());
        assert_eq!(
            dt.with_timezone(&Utc),
            NaiveDate::from_ymd_opt(2014, 11, 28)
                .unwrap()
                .and_hms_nano_opt(12, 45, 59, 324310806)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        );

        // a sample of property manipulations (validates dynamically)
        assert_eq!(dt.with_day(29).unwrap().weekday(), Weekday::Sat); // 2014-11-29 is Saturday
        assert_eq!(dt.with_day(32), None);
        assert_eq!(dt.with_year(-300).unwrap().num_days_from_ce(), -109606); // November 29, 301 BCE

        // arithmetic operations
        let dt1 = Utc.with_ymd_and_hms(2014, 11, 14, 8, 9, 10).unwrap();
        let dt2 = Utc.with_ymd_and_hms(2014, 11, 14, 10, 9, 8).unwrap();
        assert_eq!(
            dt1.signed_duration_since(dt2),
            Duration::seconds(-2 * 3600 + 2)
        );
        assert_eq!(
            dt2.signed_duration_since(dt1),
            Duration::seconds(2 * 3600 - 2)
        );
        assert_eq!(
            Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap() + Duration::seconds(1_000_000_000),
            Utc.with_ymd_and_hms(2001, 9, 9, 1, 46, 40).unwrap()
        );
        assert_eq!(
            Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap() - Duration::seconds(1_000_000_000),
            Utc.with_ymd_and_hms(1938, 4, 24, 22, 13, 20).unwrap()
        );
    }
}
