use chrono::{DateTime, Days, NaiveDate, TimeZone, Utc};

pub fn extract_starting_date(
    calendar_starting_year: i32,
    calendar_starting_week: u32,
    timezone: &impl TimeZone,
) -> Option<DateTime<Utc>> {
    let starting_date = NaiveDate::from_isoywd_opt(
        calendar_starting_year,
        calendar_starting_week as u32,
        chrono::Weekday::Mon,
    )?;

    Some(
        timezone
            .from_local_datetime(&starting_date.and_hms_opt(0, 0, 0).unwrap())
            .earliest()
            .unwrap()
            .with_timezone(&Utc),
    )
}

pub fn extract_session_date(
    starting_date: &DateTime<Utc>,
    slot_duration: u16,
    daily_slot: u16,
    session_week: u32,
    session_day: u32,
) -> Option<DateTime<Utc>> {
    let mut extracted_date =
        starting_date.checked_add_days(Days::new(((session_week - 1) * 7) as u64))?;

    extracted_date =
        extracted_date.checked_add_signed(chrono::Duration::days((session_day - 1) as i64))?;

    extracted_date = extracted_date.checked_add_signed(chrono::Duration::minutes(
        (daily_slot * slot_duration) as i64,
    ))?;

    return Some(extracted_date);
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::date_service::{extract_session_date, extract_starting_date};

    #[test]
    fn should_extract_good_starting_date() {
        assert_eq!(
            extract_starting_date(2023, 36, &Utc).unwrap(),
            Utc.with_ymd_and_hms(2023, 9, 4, 0, 0, 0).unwrap()
        )
    }

    #[test]
    fn should_extract_date_same_as_starting() {
        assert_eq!(
            extract_session_date(
                &Utc.with_ymd_and_hms(2023, 9, 4, 0, 0, 0).unwrap(),
                5,
                96,
                1,
                1,
            ),
            Some(Utc.with_ymd_and_hms(2023, 9, 4, 8, 0, 0).unwrap())
        );
    }

    #[test]
    fn should_extract_date_week_after_starting() {
        assert_eq!(
            extract_session_date(
                &Utc.with_ymd_and_hms(2023, 9, 4, 0, 0, 0).unwrap(),
                5,
                96,
                2,
                1,
            ),
            Some(Utc.with_ymd_and_hms(2023, 9, 11, 8, 0, 0).unwrap())
        );
    }

    #[test]
    fn should_extract_date_week_after_starting_different_day() {
        assert_eq!(
            extract_session_date(
                &Utc.with_ymd_and_hms(2023, 9, 4, 0, 0, 0).unwrap(),
                5,
                96,
                2,
                3,
            ),
            Some(Utc.with_ymd_and_hms(2023, 9, 13, 8, 0, 0).unwrap())
        );
    }
}
