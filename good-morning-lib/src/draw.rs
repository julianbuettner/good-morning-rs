use chrono::{Datelike, NaiveDateTime, Timelike};
use embedded_graphics::image::Image;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use ical_property::{DateMaybeTime, Event};
use tinybmp::Bmp;
use u8g2_fonts::{
    // tf end for °
    fonts::u8g2_font_fub17_tf as VERY_SMALL_FONT,
    fonts::u8g2_font_inr33_mf as SMALL_FONT,
    fonts::u8g2_font_logisoso92_tn as BIG_FONT,
    // fonts::u8g2_font_t0_22b_mr as VERY_SMALL_FONT2,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

use crate::calendar::BasicEvent;
use crate::weather::{EasyWeather, OpenMeteoPrediction};

// Big Clock and Date
const CLOCK_CENTER: i32 = 550;

const CELCIUS_CENTER: i32 = 180;

// Event Lines
const EVENT_LINE_HEIGHT: i32 = 25;

// Weather Mini Prediction
const WEATHER_PREDICTION_COUNT: usize = 6;
const MARGIN: usize = 20;
const AVAILABLE_WIDTH: usize = 800 - MARGIN;
const ELEMENT_WIDTH: usize = AVAILABLE_WIDTH / WEATHER_PREDICTION_COUNT;
const MINI_CLOCK_HEIGHT: i32 = 460;
const MINI_SYMBOL_HEIGHT: i32 = 405;
const MINI_CELCIUS_HEIGHT: i32 = 370;

pub struct DrawData {
    pub weather: OpenMeteoPrediction,
    pub datetime: NaiveDateTime,
    pub events_today: Vec<BasicEvent>,
}

const FONT_COLOR: BinaryColor = BinaryColor::Off;

impl Drawable for DrawData {
    type Color = BinaryColor;
    type Output = ();
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        // <D as DrawTarget>::Error: Debug,
    {
        let big_font = FontRenderer::new::<BIG_FONT>();
        let small_font = FontRenderer::new::<SMALL_FONT>();
        let very_small_font = FontRenderer::new::<VERY_SMALL_FONT>();
        // let very_small_font2 = FontRenderer::new::<VERY_SMALL_FONT2>();

        let mut qr = |(f, x, y, s): (&FontRenderer, i32, i32, &str)| {
            f.render_aligned(
                s,
                Point::new(x, y),
                VerticalPosition::Baseline,
                HorizontalAlignment::Center,
                FontColor::Transparent(FONT_COLOR),
                target,
            )
            .ok()
            .unwrap()
        };
        // Render Clock
        let clock = format!("{:02}:{:02}", self.datetime.hour(), self.datetime.minute());
        qr((&big_font, CLOCK_CENTER, 130, clock.as_str()));

        // Render Date
        let date = format!(
            "{:02}.{:02}.{}",
            self.datetime.day(),
            self.datetime.month(),
            self.datetime.year()
        );
        qr((&small_font, CLOCK_CENTER, 185, date.as_str()));

        // Render Celcius
        let now_celcius = self.weather.current.temperature_2m.round().to_string();
        qr((&big_font, CELCIUS_CENTER, 130, now_celcius.as_str()));
        let offset = now_celcius.len() as i32 * 33;
        qr((&small_font, CELCIUS_CENTER + 30 + offset, 130, "°C"));

        // Events
        let mut events = self.events_today.clone();
        events.sort_by_key(|e| e.time);
        let event_lines = events
            .iter()
            .map(|e| match &e.time {
                None => format!("Ganzt. {}", e.summary),
                Some(t) => format!("{:02}:{:02}  {}", t.hour(), t.minute(), e.summary)
            });
        for (i, line) in event_lines.enumerate() {
            very_small_font
                .render_aligned(
                    line.as_str(),
                    Point::new(400, 225 + i as i32 * EVENT_LINE_HEIGHT),
                    VerticalPosition::Baseline,
                    HorizontalAlignment::Left,
                    FontColor::Transparent(FONT_COLOR),
                    target,
                )
                .ok()
                .unwrap();
        }

        let mini_weather_cap = if self.events_today.len() > 5 {
            WEATHER_PREDICTION_COUNT / 2
        } else {
            WEATHER_PREDICTION_COUNT
        };
        // let now = Local::now().naive_local();
        let now = self.datetime;
        let it = self
            .weather
            .hourly
            .time
            .iter()
            .zip(self.weather.hourly.weather_code.iter())
            .zip(self.weather.hourly.temperature_2m.iter())
            .map(|((a, b), c)| (a, b, c))
            .skip_while(|(time, _, _)| time <= &&now)
            .step_by(3)
            .take(mini_weather_cap);
        let mini_x = |i: usize| i * ELEMENT_WIDTH + MARGIN / 2 + ELEMENT_WIDTH / 2;
        for (i, (time, weather_code, temperature)) in it.enumerate() {
            let time_str = format!("{:02}:{:02}", time.hour(), time.minute());
            let x: i32 = mini_x(i) as i32;
            very_small_font
                .render_aligned(
                    time_str.as_str(),
                    Point::new(x, MINI_CLOCK_HEIGHT),
                    VerticalPosition::Baseline,
                    HorizontalAlignment::Center,
                    FontColor::Transparent(FONT_COLOR),
                    target,
                )
                .ok()
                .unwrap();
            let temperature = (temperature.round() as i32).to_string();
            small_font
                .render_aligned(
                    temperature.as_str(),
                    Point::new(x, MINI_CELCIUS_HEIGHT),
                    VerticalPosition::Baseline,
                    HorizontalAlignment::Center,
                    FontColor::Transparent(FONT_COLOR),
                    target,
                )
                .ok()
                .unwrap();
            let mini_celcius_offset = temperature.len() as i32 * 10 + 12;
            very_small_font
                .render_aligned(
                    "°C",
                    Point::new(x + mini_celcius_offset, MINI_CELCIUS_HEIGHT),
                    VerticalPosition::Baseline,
                    HorizontalAlignment::Center,
                    FontColor::Transparent(FONT_COLOR),
                    target,
                )
                .ok()
                .unwrap();
            let icon = if self.weather.is_day(&time.time()) {
                weather_code.get_icon_small()
            } else {
                weather_code
                    .get_icon_small_night()
                    .unwrap_or(weather_code.get_icon_small())
            };
            let bmp = Bmp::from_slice(icon).unwrap();
            Image::with_center(&bmp, Point::new(x, MINI_SYMBOL_HEIGHT))
                .draw(target)
                .ok()
                .unwrap();
        }

        let weather_code = self.weather.current.weather_code;
        let icon = if self.weather.is_day(&self.datetime.time()) {
            weather_code.get_icon_big()
        } else {
            weather_code
                .get_icon_big_night()
                .unwrap_or(weather_code.get_icon_big())
        };
        let bmp = Bmp::from_slice(icon).unwrap();
        Image::with_center(&bmp, Point::new(200, 200))
            .draw(target)
            .ok()
            .unwrap();
        Ok(())
    }
}
