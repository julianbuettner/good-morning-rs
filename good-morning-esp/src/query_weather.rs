use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use embedded_svc::http::client::Client;
use esp_idf_svc::{
    http::client::{Configuration, EspHttpConnection},
    io::utils::try_read_full,
};
use good_morning_lib::weather::{self, OpenMeteoPrediction};

use good_morning_lib::BadMorning;

const BUFFER_SIZE_KB: usize = 32;

// To calulate minutes to decimal, divide by 60, multiply by 100
const LONG: &str = dotenv!("LONG"); // East / West
const LAT: &str = dotenv!("LAT"); // North / South
const TIMEZONE: &str = dotenv!("TIMEZONE");  // E.g. "Europe/Berlin"

pub fn query_weather() -> Result<(OpenMeteoPrediction, DateTime<Utc>), BadMorning> {
    let mut client = Client::wrap(
        EspHttpConnection::new(&Configuration {
            crt_bundle_attach: Some(esp_idf_hal::sys::esp_crt_bundle_attach),
            ..Default::default()
        })
        .unwrap(),
    );

    let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE_KB * 1024];
    let long: f32 = LONG.parse().unwrap();
    let lat: f32 = LAT.parse().unwrap();
    let url = weather::get_http_url(long, lat, TIMEZONE);
    println!("Weather url: {}", url);
    let request = client.get(&url).unwrap();
    let mut response = request.submit().unwrap();
    let header_date = response.header("Date").unwrap();
    let parsed_date = DateTime::parse_from_rfc2822(header_date)
        .expect("Failed to parse date RFC2822")
        .with_timezone(&Utc); // Convert to Utc
    let read = try_read_full(&mut response, &mut buffer).map_err(|_| BadMorning::HttpConnection)?;
    println!("Weather bytes read: {}", read);
    let body = String::from_utf8_lossy(&buffer[..read]).into_owned();
    println!("Body: {}", body);
    Ok((weather::parse_meteo(&body)?, parsed_date))
}
