mod codes;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};
use codes::WeatherCode;
use serde::{de, Deserialize, Deserializer};

use crate::error::BadMorning;

#[derive(Debug, Deserialize)]
pub struct Current {
    pub temperature_2m: f32,
    pub weather_code: WeatherCode,
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize a vector of strings, then parse each string into NaiveDateTime
    let string: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&string, "%Y-%m-%dT%H:%M")
        .map_err(|e| de::Error::custom(format!("Invalid datetime: {}: {}", string, e)))
}

pub fn deserialize_datetime_vec<'de, D>(deserializer: D) -> Result<Vec<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize a vector of strings, then parse each string into NaiveDateTime
    let strings: Vec<String> = Deserialize::deserialize(deserializer)?;
    strings
        .into_iter()
        .map(|s| {
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M")
                .map_err(|e| de::Error::custom(format!("Invalid datetime: {}: {}", s, e)))
        })
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct Hourly {
    #[serde(deserialize_with = "deserialize_datetime_vec")]
    pub time: Vec<NaiveDateTime>,
    pub temperature_2m: Vec<f32>,
    pub weather_code: Vec<WeatherCode>,
}
#[derive(Debug, Deserialize)]
pub struct Daily {
    pub time: Vec<NaiveDate>,
    #[serde(deserialize_with = "deserialize_datetime_vec")]
    pub sunrise: Vec<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_datetime_vec")]
    pub sunset: Vec<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct OpenMeteoPrediction {
    pub current: Current,
    pub hourly: Hourly,
    pub daily: Daily,
}

pub fn get_http_url(long: f32, lat: f32, timezone: &str) -> String {
    format!(
        "https://api.open-meteo.com/v1/forecast?\
        latitude={}&\
        longitude={}&\
        current=temperature_2m,weather_code&\
        hourly=temperature_2m,weather_code&\
        daily=sunrise,sunset&\
        timezone={}&\
        forecast_days=2",
        lat,
        long,
        timezone.replace("/", "%2F"), // e.g. Europe/Berlin
    )
}

pub fn parse_meteo(body: &str) -> Result<OpenMeteoPrediction, BadMorning> {
    serde_json::from_str(body).map_err(|e| BadMorning::MetoResponseFormat(e.to_string()))
}

pub trait EasyWeather {
    fn sunrise(&self) -> NaiveTime;
    fn sunset(&self) -> NaiveTime;
    fn is_day(&self, time: &NaiveTime) -> bool {
        // Assuming sun rises once after 00:00 and sets before 23:59.
        // Don't use if in polar circle.
        (self.sunrise()..self.sunset()).contains(time)
    }
}

impl EasyWeather for OpenMeteoPrediction {
    fn sunrise(&self) -> NaiveTime {
        self.daily.sunrise.first().unwrap().time()
    }
    fn sunset(&self) -> NaiveTime {
        self.daily.sunset.first().unwrap().time()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_example_weather_response() {
        let weather_response = include_str!("../resources/example-weather.json");
        let _meteo: OpenMeteoPrediction = serde_json::from_str(weather_response).unwrap();
    }
}
