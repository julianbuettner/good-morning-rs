use std::{borrow::Borrow, sync::Arc, thread::sleep, time::Duration};

use chrono::{
    Date, DateTime, Duration as ChronoDur, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc
};
use chrono_tz::{Europe::Berlin, Tz};
// use display::display_something;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{BinaryColor, Rgb565, Rgb666},
    prelude::{DrawTarget, *},
    text::Text,
};
use epd_waveshare::{
    epd7in5_v2::{Display7in5, Epd7in5},
    prelude::WaveshareDisplay,
};
use esp_idf_hal::{
    delay::Delay,
    gpio::{AnyOutputPin, Gpio0, OutputPin, Pin, PinDriver},
    peripheral::Peripheral,
    prelude::Peripherals,
    spi::{
        self, config::Config, SpiBusDriver, SpiDeviceDriver, SpiDriver, SpiDriverConfig,
        SpiSingleDeviceDriver,
    },
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    sntp::{EspSntp, SyncStatus},
};
use good_morning_lib::{calendar::BasicEvent, BadMorning, DrawData};
use ical_property::{DateMaybeTime, Event};
use query_weather::query_weather;
use time::{time_from_internet, TimeSync};

// mod display;
mod query_calendar;
mod query_weather;
mod time;
mod wifi;

const TZ: Tz = Berlin;
const REFRESH_RATE: i64 = 300;

fn get_refresh_rate(now: &DateTime<Utc>) -> Duration {
    let now = now.naive_local().time();
    let h = |h| NaiveTime::from_hms_opt(h, 0, 0).unwrap();
    if (h(0)..h(6)).contains(&now) {
        return Duration::from_secs(1800);
    }
    Duration::from_secs(300)
}

fn blink(pin: impl OutputPin) {
    let mut d = PinDriver::output(pin).unwrap();
    for i in 0..8 {
        d.toggle().unwrap();
        sleep(Duration::from_millis(250));
    }
}

fn duration_to_next_refresh(now: DateTime<Utc>) -> Duration {
    let refresh_rate_sec = get_refresh_rate(&now).as_secs() as i64;
    let now_sec = now.timestamp();
    let next_sec = (now_sec / refresh_rate_sec) * refresh_rate_sec + refresh_rate_sec;
    let next: DateTime<Utc> = DateTime::from_timestamp(next_sec, 0).unwrap();
    println!("Cooldown {}", refresh_rate_sec);
    println!("Now {:?}", now.timestamp());
    println!("Now {:?}", now);
    println!("Sleep until {}", next_sec);
    println!("Sleep until {:?}", next);
    let dif = next - now;
    let res = Duration::from_secs(dif.num_seconds() as u64);
    println!("Sleep {}s", res.as_secs());
    res
}

fn routine() -> Result<(), BadMorning> {
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    // https://www.waveshare.com/wiki/E-Paper_ESP32_Driver_Board
    let din = peripherals.pins.gpio14;
    let sclk = peripherals.pins.gpio13;
    let cs = peripherals.pins.gpio15;
    let dc = peripherals.pins.gpio27;
    let rst = peripherals.pins.gpio26;
    let busy = peripherals.pins.gpio25;

    eprintln!("Init SPI Bus");
    let spi = peripherals.spi2;
    let sdi: Option<Gpio0> = None;
    let sdo = din;
    let config: SpiDriverConfig = SpiDriverConfig::default();
    let spi_driver =
        SpiDriver::new(spi, sclk, sdo, sdi, &config).expect("Could not create SpiDriver");

    eprintln!("Init SPI Device");
    let spi_device_config: Config = Default::default();
    let mut spi_device_driver =
        SpiDeviceDriver::new(spi_driver, Some(cs), &spi_device_config).unwrap();

    let busy = PinDriver::input(busy).unwrap();
    let dc = PinDriver::output(dc).unwrap();
    let rst = PinDriver::output(rst).unwrap();
    let delay_us = None;
    let mut delay = Delay::new_default();
    let mut epd =
        Epd7in5::new(&mut spi_device_driver, busy, dc, rst, &mut delay, delay_us).unwrap();
    // Datetime
    eprintln!("Connect to wifi.");
    let timeout = Duration::from_secs(15);
    let wifi_handle =
        wifi::connect_to_wifi_with_timeout(timeout, peripherals.modem, sys_loop, nvs)?;
    sleep(Duration::from_secs(1));
    // Start Time Sync
    eprintln!("Sync time.");
    let time_sync = TimeSync::new()?;

    let sl = |s| {
        eprintln!("Sleep {}s", s);
        sleep(Duration::from_secs(s));
    };

    loop {
        let mut display_raw = Box::new(Display7in5::default());
        eprintln!("Fetch weather with datetime...");
        let (weather, now) = query_weather()?;
        eprintln!(
            "Weather and datetime fetched. Temp: {}°C, now {}",
            weather.current.temperature_2m, now,
        );
        let events_today = query_calendar::get_events().unwrap_or(vec![BasicEvent {
            summary: "Kalender reparieren".to_string(),
            time: None,
        }]);
        let draw_data = DrawData {
            events_today,
            weather,
            datetime: now.with_timezone(&Berlin).naive_local(),
        };

        eprintln!("Draw something to buffer.");
        draw_data
            .draw(&mut display_raw.color_converted::<BinaryColor>())
            .unwrap();
        sl(1);

        eprintln!("Update SRM.");
        epd.update_frame(&mut spi_device_driver, display_raw.buffer(), &mut delay)
            .expect("Failed to print to screen :(");
        sl(1);

        eprintln!("Display SRAM");
        epd.display_frame(&mut spi_device_driver, &mut delay)
            .expect("Failed to print to screen :(");
        sl(1);

        eprintln!("Sleep EPD");
        epd.sleep(&mut spi_device_driver, &mut delay).unwrap();

        let mut sleep_duration = duration_to_next_refresh(now);
        if sleep_duration < Duration::from_secs(60) {
            println!("Sleep extra {}s", REFRESH_RATE);
            sleep_duration += Duration::from_secs(REFRESH_RATE as u64)
        }
        sleep(sleep_duration - Duration::from_secs(3));

        eprintln!("Wake Up");
        epd.wake_up(&mut spi_device_driver, &mut delay).unwrap();
    }
    eprintln!("Clear Frame.");
    epd.clear_frame(&mut spi_device_driver, &mut delay).unwrap();
    epd.wait_until_idle(&mut spi_device_driver, &mut delay)
        .unwrap();
    eprintln!("Sleep Display.");
    epd.sleep(&mut spi_device_driver, &mut delay).unwrap();

    drop(wifi_handle);
    drop(time_sync);
    Ok(())
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    routine().expect("Routine failed");
    println!("Done. Sleep.");
    sleep(Duration::from_secs(180));
}
