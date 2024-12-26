use serde::Deserialize;

use crate::BadMorning;


#[derive(Debug, Deserialize)]
pub struct WorldTimeApiResponse {
    pub unixtime: i64,
}

pub fn parse_world_time(body: &str) -> Result<WorldTimeApiResponse, BadMorning> {
    serde_json::from_str(body).map_err(|e| BadMorning::MetoResponseFormat(e.to_string()))
}
pub fn get_http_url() -> &'static str {
    &"https://worldtimeapi.org/api/timezone/Europe/Berlin"
}


