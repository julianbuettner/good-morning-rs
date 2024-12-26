use std::io::BufReader;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use dotenvy_macro::dotenv;
use embedded_svc::http::client::Client;
use esp_idf_hal::io::BufRead;
use esp_idf_svc::{
    http::client::{Configuration, EspHttpConnection},
    io::utils::try_read_full,
};
use good_morning_lib::BadMorning;
use ical_property::Event;

const CALENDAR_URL: &str = dotenv!("CALENDAR_URL");
const BUFFER_SIZE_KB: usize = 200; // Private Calendar 166KiB Dec 2024

// inside embedded reader, outside std::io reader
struct ReadMapper<R: esp_idf_svc::io::Read> {
    inner: R,
}

impl<R: esp_idf_svc::io::Read> ReadMapper<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: esp_idf_svc::io::Read> std::io::Read for ReadMapper<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner
            .read(buf)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Unsupported, "Oh oh"))
    }
}

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
    let request = client.get(&CALENDAR_URL).unwrap();
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
