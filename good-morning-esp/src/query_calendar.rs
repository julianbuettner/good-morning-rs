use std::io::BufReader;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use dotenvy_macro::dotenv;
use embedded_svc::http::client::Client;
use esp_idf_hal::io::{BufRead, Write};
use esp_idf_svc::{
    http::client::{Configuration, EspHttpConnection},
    io::utils::try_read_full,
};
use good_morning_lib::BadMorning;
use ical_property::Event;

const CALENDAR_URL: &str = dotenv!("CALENDAR_URL");
const CALENDAR_PROXY_URL: &str = dotenv!("CALENDAR_PROXY");
const BUFFER_SIZE_KB: usize = 4;

pub fn get_events() -> Result<Vec<Event>, BadMorning> {
    let mut client = Client::wrap(
        EspHttpConnection::new(&Configuration {
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            ..Default::default()
        })
        .unwrap(),
    );

    // let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE_KB * 1024];
    println!("Calendar url: {}", CALENDAR_URL);
    println!("Calendar proxy: {}", CALENDAR_PROXY_URL);
    let request = client
        .post(&CALENDAR_PROXY_URL, &[])
        .ok_or(BadMorning::CalendarProxyOffline)?
        .write_all(CALENDAR_URL.as_bytes())
        .ok_or(BadMorning::CalendarProxyOffline)?
        .flush()
        .ok_or(BadMorning::CalendarProxyOffline)?
    ;
    let mut response = request.submit().unwrap();
    // let read = try_read_full(&mut response, &mut buffer).map_err(|_| BadMorning::HttpConnection)?;
    // println!("Calendar Bytes read: {}", read);
    // let body = String::from_utf8_lossy(&buffer[..read]).into_owned();
    let read_mapper = ReadMapper::new(response);
    let mut buf_reader = BufReader::new(read_mapper);
    let all_events = ical::IcalParser::new(buf_reader)
        .map(|e| e.expect("Could not parse entry"))
        .flat_map(|cal| cal.events);
    let all_events = good_morning_lib::calendar::filter_map_events(all_events);
    Ok(good_morning_lib::calendar::events_at_date(
        all_events,
        Local::now().date_naive(),
    ))
}
