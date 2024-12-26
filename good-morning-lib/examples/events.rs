use chrono::NaiveDate;
use good_morning_lib::calendar::events_at_date;
use ical_property::Event;

fn main() {
    let uk: &[u8] = include_bytes!("../private.ics");
    let events = ical::IcalParser::new(uk)
        .map(|e| e.expect("Could not parse entry"))
        .flat_map(|cal| cal.events)
        .map(Event::try_from)
        .map(|e| e.expect("Could not convert into Event"));
    let date: NaiveDate = NaiveDate::from_ymd_opt(2029, 10, 29).unwrap();
    for event in events_at_date(events, date) {
        println!("{:?}", event);
    }

}
