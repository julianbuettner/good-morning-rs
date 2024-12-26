use std::io::BufReader;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use dotenvy_macro::dotenv;
use embedded_svc::http::client::Client;
use esp_idf_hal::io::{BufRead, Write};
use esp_idf_svc::{
    http::client::{Configuration, EspHttpConnection},
    io::utils::try_read_full,
};
use good_morning_lib::{calendar::BasicEvent, BadMorning};
use ical_property::Event;

const CALENDAR_URL: &str = dotenv!("CALENDAR_URL");
const CALENDAR_PROXY_URL: &str = dotenv!("CALENDAR_PROXY");
const BUFFER_SIZE_KB: usize = 4;

pub fn get_events() -> Result<Vec<BasicEvent>, BadMorning> {
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
    let mut request = client
        .post(&CALENDAR_PROXY_URL, &[])
        .ok()
        .ok_or(BadMorning::CalendarProxyOffline)?;
    request
        .write_all(CALENDAR_URL.as_bytes())
        .ok()
        .ok_or(BadMorning::CalendarProxyOffline)?;
    request
        .flush()
        .ok()
        .ok_or(BadMorning::CalendarProxyOffline)?;
    let mut response = request.submit().unwrap();
    let mut buffer = vec![0u8; BUFFER_SIZE_KB];
    let read = try_read_full(&mut response, &mut buffer).map_err(|_| BadMorning::HttpConnection)?;
    let response_body = String::from_utf8_lossy(&buffer[..read]).into_owned();
    let calendar_result: Result<Vec<BasicEvent>, BadMorning> = serde_json::from_str(&response_body)
        .ok()
        .ok_or(BadMorning::CalendarProxyUnexpected);
    if let Err(e) = &calendar_result {
        println!("Calendar fetching, error from server: {:?}", e);
    }
    calendar_result
}
