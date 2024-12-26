use std::usize;

use chrono::{DateTime, NaiveDate, NaiveTime};
use ical::parser::ical::component::IcalEvent;
use ical_property::{DateMaybeTime, Event};
use serde::{Deserialize, Serialize};

const MAX_EVENT_ITER: usize = 10_000_000;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicEvent {
    pub summary: String,
    pub time: Option<NaiveTime>,
}

pub fn events_at_date(events: impl Iterator<Item = Event>, date: NaiveDate) -> Vec<Event> {
    let mut result = Vec::new();
    for event in events {
        if event.rrule.is_some() {
            // println!(
            //     "Check RR event {} at {:?}",
            //     event.summary.as_ref().unwrap(),
            //     event.start.as_ref().unwrap()
            // );
            for start in rr_event_starts_at_date(&event, date) {
                println!("Keep event {}", event.summary.as_ref().unwrap());
                result.push(Event {
                    summary: event.summary.clone(),
                    start: Some(DateMaybeTime::DateTime(start.to_utc())),
                    ..Default::default()
                });
            }
        } else {
            // println!(
            //     "Check OT event {} at {:?}",
            //     event.summary.as_ref().unwrap(),
            //     event.start.as_ref().unwrap()
            // );
            if event_at_date(&event, date) {
                println!("Keep event {}", event.summary.as_ref().unwrap());
                result.push(event.clone());
            }
        }
    }
    result
}

pub fn event_at_date(event: &Event, date: NaiveDate) -> bool {
    assert!(event.rrule.is_none());
    match (event.start.as_ref(), event.end.as_ref()) {
        (Some(a), Some(b)) => (a.as_naive_date()..=b.as_naive_date()).contains(&date),
        (Some(a), None) => a.as_naive_date() <= date,
        (None, Some(b)) => b.as_naive_date() >= date,
        // This makes no sense. Avoid panic.
        (None, None) => false,
    }
}

pub fn rr_event_starts_at_date(
    event: &Event,
    date: NaiveDate,
) -> impl Iterator<Item = DateTime<rrule::Tz>> {
    let date2 = date.clone();
    event
        .rrule
        .as_ref()
        .expect("Call this only for events with RRule")
        .into_iter()
        .take(MAX_EVENT_ITER)
        .take_while(move |event| event.naive_local().date() <= date)
        .skip_while(move |event| event.naive_local().date() < date2)
}

pub fn filter_map_events(events: impl Iterator<Item = IcalEvent>) -> impl Iterator<Item = Event> {
    events.map(|e| (&e).try_into().unwrap())
}
