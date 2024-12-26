use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::{DateTime, Local, NaiveDate, Utc};
use embedded_svc::http::client::Client;
use esp_idf_svc::{http::client::{Configuration, EspHttpConnection}, io::utils::try_read_full, sntp::{EspSntp, SyncStatus}};
use good_morning_lib::{datetime, BadMorning};

pub struct TimeSync<'a> {
    inner: EspSntp<'a>,
}

impl<'a> TimeSync<'a> {
    pub fn new() -> Result<Self, BadMorning> {
        let inner = EspSntp::new_default().map_err(|_| BadMorning::SntpError)?;
        eprintln!("Esp at creation {:?}", inner.get_sync_status());
        Ok(Self { inner })
    }

    pub fn block_timeout(&self, timeout: Duration) -> Result<(), BadMorning> {
        let start = Instant::now();
        while start.elapsed() <= timeout {
            let status = self.inner.get_sync_status();
            eprintln!("{:?} with dt {:?}", status, Local::now());
            if status == SyncStatus::Completed {
                return Ok(());
            }
            if Local::now().date_naive() > NaiveDate::from_ymd_opt(2024, 12, 01).unwrap() {
                return Ok(());
            }
            sleep(Duration::from_millis(2000));
        }
        Err(BadMorning::SntpTimeout)
    }
}

pub fn time_from_internet() -> Result<DateTime<Utc>, BadMorning> {
    let url = datetime::get_http_url();
    let mut client = Client::wrap(
        EspHttpConnection::new(&Configuration {
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            ..Default::default()
        })
        .unwrap(),
    );
    let mut buffer: Vec<u8> = vec![0; 8 * 1024];
    let request = client.get(&url).unwrap();
    let mut response = request.submit().unwrap();
    let read = try_read_full(&mut response, &mut buffer).map_err(|_| BadMorning::HttpConnection)?;
    let body = String::from_utf8_lossy(&buffer[..read]).into_owned();
    let fetched = datetime::parse_world_time(&body)?;
    Ok(DateTime::from_timestamp(fetched.unixtime, 0).unwrap())
}
