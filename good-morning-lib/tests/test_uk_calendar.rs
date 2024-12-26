use std::io::{BufRead, BufReader, Read};

use chrono::NaiveDate;
use ical::parser::ical;
use ical_property::Event;

#[test]
fn uk_calendar_christmas() {
    let uk: &[u8] = include_bytes!("./UK_Holidays.ics");
    let mut events = ical::IcalParser::new(uk)
        .map(|e| e.expect("Could not parse entry"))
        .flat_map(|cal| cal.events)
        .map(Event::try_from)
        .map(|e| e.expect("Could not convert into Event"));

    let christmas = events
        .find(|e| e.summary == Some("Christmas Day".to_string()))
        .expect("There is a year without christmas?");
    assert_eq!(
        christmas.start.unwrap().as_naive_date(),
        NaiveDate::from_ymd_opt(2024, 12, 25).unwrap()
    );
}
