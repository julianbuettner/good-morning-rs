use good_morning_lib::weather::{get_http_url, parse_meteo};

// Please run this test less often than 10K times per day.
// (Averages to every 8.64s)

#[test]
fn fetch_and_parse_open_meteo() {
    let url = get_http_url(48., 9., "Europe/Berlin");
    let body = reqwest::blocking::get(&url)
        .expect("Failed to reqwest Open Meteo")
        .text()
        .expect("Failed to get Open Meteo Body");
    parse_meteo(&body).expect("Failed to parse Open Meteo Response");
}
