use std::{fmt::Binary, str::FromStr};

use chrono::{DateTime, NaiveDate, Utc};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use good_morning_lib::{
    weather::{get_http_url, parse_meteo},
    DrawData,
};
use ical_property::{DateMaybeTime, Event};
use u8g2_fonts::{
    fonts::u8g2_font_logisoso92_tn as FONT,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer, U8g2TextStyle,
};

// Fine with size and style: u8g2_font_inb33_mr
// Fine with size and style: u8g2_font_inb56_mr

fn get_events() -> Vec<Event> {
    vec![
        Event {
            summary: Some("Kaffe und Kuchen".to_string()),
            start: Some(DateMaybeTime::DateTime(
                "2012-12-12T14:30:12Z".parse::<DateTime<Utc>>().unwrap(),
            )),
            ..Default::default()
        },
        Event {
            summary: Some("Weihnachten".into()),
            start: Some(DateMaybeTime::Date(
                NaiveDate::from_str("2023-12-12").unwrap(),
            )),
            ..Default::default()
        },
        Event {
            summary: Some("Zahnarzt Praxis Herbst".to_string()),
            start: Some(DateMaybeTime::DateTime(
                "2012-12-12T11:45:12Z".parse::<DateTime<Utc>>().unwrap(),
            )),
            ..Default::default()
        },
        Event {
            summary: Some("Geburtstag Oma".into()),
            start: Some(DateMaybeTime::Date(
                NaiveDate::from_str("2023-12-12").unwrap(),
            )),
            ..Default::default()
        },
    ]
}

fn main() -> Result<(), core::convert::Infallible> {
    let url = get_http_url(48., 9., "Europe/Berlin");
    let body = reqwest::blocking::get(&url)
        .expect("Failed to reqwest Open Meteo")
        .text()
        .expect("Failed to get Open Meteo Body");
    // let body = include_str!("../resources/example-weather.json");
    eprintln!("Meteo {}", body);
    let weather = parse_meteo(&body).expect("Failed to parse Open Meteo Response");

    let draw_data = DrawData {
        weather,
        datetime: chrono::Local::now().naive_local(),
        events_today: get_events(),
    };

    let mut display =
        SimulatorDisplay::<BinaryColor>::with_default_color(Size::new(800, 480), BinaryColor::On);
    draw_data.draw(&mut display).unwrap();
    let output_settings = OutputSettingsBuilder::new()
        .scale(1)
        .theme(BinaryColorTheme::OledBlue)
        .build();

    Window::new("Hello World", &output_settings).show_static(&display);
    Ok(())
}
