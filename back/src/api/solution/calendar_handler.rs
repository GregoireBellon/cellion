use std::{str::FromStr, usize};

use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use log::warn;

use super::xml_types::XmlCalendar;

pub struct CalendarHandler {
    pub starting_date: NaiveDateTime,
    pub slot_duration: u16,
    sequence_week_association_table: Option<Vec<u32>>,
}

impl CalendarHandler {
    pub fn new() -> Self {
        CalendarHandler {
            starting_date: Utc
                .with_ymd_and_hms(Local::now().year(), 1, 1, 0, 0, 0)
                .unwrap()
                .naive_local(),

            slot_duration: 1,
            sequence_week_association_table: None,
        }
    }

    pub fn register_xml_calendar(&mut self, xml_calendar: &XmlCalendar) {
        // Minutes in a day divided by the number of slots in the day
        self.slot_duration = ((60 * 24) / xml_calendar.slots.nr) as u16;

        let extracted_date = extract_starting_date(xml_calendar.year, xml_calendar.starting_week);

        match extracted_date {
            Some(extracted) => self.starting_date = extracted,
            None => warn!("The starting date described by calendar is not valid !"),
        }

        if let Some(week_seq) = xml_calendar.weeks.sequence.as_ref() {
            self.sequence_week_association_table = parse_str_seq_to_association_table(
                week_seq.as_str(),
                xml_calendar.weeks.nr as usize,
            )
            .inspect_err(|_| warn!("The sequence week format seem invalid"))
            .ok();
        }
    }

    fn get_delta_week(&self, week: u32) -> u32 {
        self.sequence_week_association_table
            .as_ref()
            .and_then(|table| table.get((week - 1) as usize).copied())
            .unwrap_or(week)
            - 1
    }

    pub fn extract_session_date(
        &self,
        daily_slot: u16,
        session_week: u32,
        session_day: u32,
    ) -> Option<NaiveDateTime> {
        let mut extracted_date = self
            .starting_date
            .checked_add_days(Days::new((self.get_delta_week(session_week) * 7) as u64))?;

        extracted_date =
            extracted_date.checked_add_signed(chrono::Duration::days((session_day - 1) as i64))?;

        extracted_date = extracted_date.checked_add_signed(chrono::Duration::minutes(
            (daily_slot * self.slot_duration) as i64,
        ))?;

        return Some(extracted_date);
    }
}

/// Extracts the calendar starting date from the starting year and starting week
pub fn extract_starting_date(
    calendar_starting_year: i32,
    calendar_starting_week: u32,
) -> Option<NaiveDateTime> {
    let starting_date = NaiveDate::from_isoywd_opt(
        calendar_starting_year,
        calendar_starting_week as u32,
        chrono::Weekday::Mon,
    )?;

    return Some(NaiveDateTime::new(
        starting_date,
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    ));
}

#[derive(PartialEq, Debug)]
pub enum Sequence<T> {
    Elem(T),
    Range(std::ops::Range<T>),
}

/// Parse a string which has the form of \"T-T\"
fn parse_range_sequence<T>(str_range: &str) -> Result<Sequence<T>, ()>
where
    T: FromStr,
{
    let mut splitted_elem = str_range.split('-');
    let first_elem = splitted_elem.next();
    let second_elem = splitted_elem.next();
    let third_elem = splitted_elem.next();

    match (first_elem, second_elem, third_elem) {
        // ensure that the range has ONLY 2 elements
        (Some(first), Some(second), None) => match (first.parse::<T>(), second.parse::<T>()) {
            // parse the 2 elements
            (Ok(range1), Ok(range2)) => Ok(Sequence::Range(std::ops::Range {
                start: range1,
                end: range2,
            })),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn parse_str_to_sequence<T>(seq: &str) -> impl Iterator<Item = Result<Sequence<T>, ()>> + '_
where
    T: FromStr,
{
    let splitted_seq = seq.split(',');

    return splitted_seq.map(|split| match split.contains('-') {
        true => parse_range_sequence(split),
        false => match split.parse::<T>() {
            Ok(parsed_elem) => Ok(Sequence::Elem(parsed_elem)),
            Err(_) => Err(()),
        },
    });
}

fn create_sequence_association_table(
    seq_size: usize,
    seq: impl Iterator<Item = Sequence<u32>>,
) -> Vec<u32> {
    let mut result = Vec::with_capacity(seq_size);

    let mut cursor: usize = 0;

    for item in seq {
        match item {
            Sequence::Elem(e) => {
                result.insert(cursor, e);
                cursor += 1;
            }
            Sequence::Range(r) => {
                for n in r {
                    result.insert(cursor, n);
                    cursor += 1;
                }
            }
        }
    }

    return result;
}

pub fn parse_str_seq_to_association_table(seq: &str, seq_size: usize) -> Result<Vec<u32>, ()> {
    let mut parsing_error = false;

    let parsed_seq = parse_str_to_sequence::<u32>(seq)
        .into_iter()
        .inspect(|r| {
            if r.is_err() {
                parsing_error = true;
            }
        })
        .filter_map(Result::ok);

    let assoc_table = create_sequence_association_table(seq_size, parsed_seq);

    return if parsing_error {
        Err(())
    } else {
        Ok(assoc_table)
    };
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::solution::calendar_handler::{
        create_sequence_association_table, extract_starting_date,
        parse_str_seq_to_association_table, parse_str_to_sequence, CalendarHandler,
        Sequence::{self, Elem, Range},
    };

    #[test]
    fn should_extract_good_starting_date() {
        assert_eq!(
            extract_starting_date(2023, 36).unwrap(),
            Utc.with_ymd_and_hms(2023, 9, 4, 0, 0, 0)
                .unwrap()
                .naive_local()
        )
    }

    #[test]
    fn should_extract_date_same_as_starting() {
        let mut calendar = CalendarHandler::new();
        calendar.starting_date = Utc
            .with_ymd_and_hms(2023, 9, 4, 0, 0, 0)
            .unwrap()
            .naive_local();

        assert_eq!(
            calendar.extract_session_date(480, 1, 1),
            Some(
                Utc.with_ymd_and_hms(2023, 9, 4, 8, 0, 0)
                    .unwrap()
                    .naive_local()
            )
        );
    }

    #[test]
    fn should_extract_date_week_after_starting() {
        let mut calendar = CalendarHandler::new();
        calendar.starting_date = Utc
            .with_ymd_and_hms(2023, 9, 4, 0, 0, 0)
            .unwrap()
            .naive_local();
        assert_eq!(
            calendar.extract_session_date(480, 2, 1,),
            Some(
                Utc.with_ymd_and_hms(2023, 9, 11, 8, 0, 0)
                    .unwrap()
                    .naive_local()
            )
        );
    }

    #[test]
    fn should_extract_date_week_after_starting_different_day() {
        let mut calendar = CalendarHandler::new();
        calendar.starting_date = Utc
            .with_ymd_and_hms(2023, 9, 4, 0, 0, 0)
            .unwrap()
            .naive_local();

        assert_eq!(
            calendar.extract_session_date(480, 2, 3),
            Some(
                Utc.with_ymd_and_hms(2023, 9, 13, 8, 0, 0)
                    .unwrap()
                    .naive_local()
            )
        );
    }

    #[test]
    fn should_extract_date_with_gap_week() {
        let mut calendar = CalendarHandler::new();
        calendar.starting_date = Utc
            .with_ymd_and_hms(2023, 9, 4, 0, 0, 0)
            .unwrap()
            .naive_local();
        calendar.sequence_week_association_table = Some(vec![1, 2, 4]);

        assert_eq!(
            calendar.extract_session_date(480, 3, 1),
            Some(
                Utc.with_ymd_and_hms(2023, 9, 25, 8, 0, 0)
                    .unwrap()
                    .naive_local()
            )
        );
    }

    #[test]
    fn should_parse_str_to_sequence_type_no_range() {
        assert_eq!(
            parse_str_to_sequence::<u32>("1,2,3,4,5")
                .filter_map(Result::ok)
                .collect::<Vec<Sequence<u32>>>(),
            vec![Elem(1), Elem(2), Elem(3), Elem(4), Elem(5)]
        )
    }

    #[test]
    fn should_parse_str_to_sequence_type_with_range() {
        assert_eq!(
            parse_str_to_sequence("1,2,3,4-12")
                .filter_map(Result::ok)
                .collect::<Vec<Sequence<u32>>>(),
            vec![
                Elem(1),
                Elem(2),
                Elem(3),
                Range(std::ops::Range { start: 4, end: 12 })
            ]
        )
    }

    #[test]
    fn should_create_sequence_association_table_no_range() {
        assert_eq!(
            create_sequence_association_table(
                5,
                vec![Elem(1), Elem(3), Elem(7), Elem(8), Elem(12)].into_iter()
            ),
            vec![1, 3, 7, 8, 12]
        )
    }

    #[test]
    fn should_create_sequence_association_table_with_range() {
        assert_eq!(
            create_sequence_association_table(
                9,
                vec![
                    Elem(1),
                    Range(std::ops::Range { start: 3, end: 7 }),
                    Elem(8),
                    Range(std::ops::Range { start: 9, end: 10 })
                ]
                .into_iter()
            ),
            vec![1, 3, 4, 5, 6, 8, 9]
        )
    }

    #[test]
    fn sould_map_session_with_sequence() {
        let mut calendar = CalendarHandler::new();

        calendar.starting_date = Utc
            .with_ymd_and_hms(2023, 9, 4, 0, 0, 0)
            .unwrap()
            .naive_local();
        calendar.sequence_week_association_table =
            parse_str_seq_to_association_table("1,2,4,5,7-27", 24).ok();

        assert!(calendar.sequence_week_association_table.is_some());

        assert_eq!(
            calendar.extract_session_date(480, 3, 1,),
            Some(
                Utc.with_ymd_and_hms(2023, 9, 25, 8, 0, 0)
                    .unwrap()
                    .naive_local()
            )
        );
    }
}
