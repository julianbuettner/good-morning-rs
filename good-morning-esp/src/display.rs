use std::{thread::sleep, time::Duration};

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::Point,
    text::{Baseline, Text, TextStyleBuilder},
    Drawable,
};
use embedded_hal::{
    digital::{InputPin, OutputPin},
    spi::SpiDevice,
};
use epd_waveshare::{
    color::Color,
    epd7in5_v2::{Display7in5, Epd7in5},
    prelude::WaveshareDisplay,
};
use esp_idf_hal::{
    delay::Delay,
    gpio::{AnyInputPin, AnyOutputPin},
};

pub struct Display75<SPI, BUSY, DC, RST>
where
    SPI: SpiDevice,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
{
    spi: SPI,
    delay: Delay,
}

impl<SPI, BUSY, DC, RST> Display75<SPI, BUSY, DC, RST> {
    pub fn new(spi: SPI, busy: AnyInputPin, dc: AnyOutputPin, rst: AnyOutputPin) -> Self {
        let mut delay = Delay::new_default();
        let epd = Epd7in5::new(&mut spi, busy, dc, rst, &mut delay, None).unwrap();
        todo!()
    }
    fn flush(&mut self) {}
}

pub fn display_something(
    spi: &mut impl SpiDevice,
    busy: impl InputPin,
    dc: impl OutputPin,
    rst: impl OutputPin,
) {
    eprintln!("What?");
    let mut delay = Delay::new_default();
    eprintln!("What 2");
    let delay_us = None;
    let mut epd = Epd7in5::new(spi, busy, dc, rst, &mut delay, delay_us).unwrap();
    eprintln!("What 3");
    let mut display = Box::new(Display7in5::default());
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(Color::White)
        .background_color(Color::Black)
        .build();
    eprintln!("What 4");
    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    sleep(Duration::from_millis(1));
    eprintln!("What 5");
    // Draw some text at a certain point using the specified text style
    Text::with_text_style("It's working-WoB!", Point::new(100, 100), style, text_style)
        .draw(&mut *display)
        .unwrap();
    Text::with_text_style("It's not working!", Point::new(200, 200), style, text_style)
        .draw(&mut *display)
        .unwrap();
    eprintln!("Wake up");
    epd.wake_up(spi, &mut delay).expect("Failed waking up");
    eprintln!("BG col");
    epd.set_background_color(Color::Black);
    eprintln!("Clear frame");
    epd.clear_frame(spi, &mut delay)
        .expect("Clear Frame failed");
    eprintln!("Buf");
    epd.update_and_display_frame(spi, display.buffer(), &mut delay)
        .expect("Failed to print to screen :(");
    // epd.update_frame(spi, display.buffer(), &mut delay).unwrap();
    // epd.display_frame(spi, &mut delay)
    //     .expect("display frame new graphics");
    eprintln!("What 6");
    // Draw some text at a certain point using the specified text style
}

// fn meh() {
//     eprintln!("{}", 1);
//     let mut delay = Delay::new_default();
//     eprintln!("{}", 2);
//     let delay_us = None;
//     eprintln!("{}", 3);
//     let mut epd = Epd7in5::new(spi, busy, dc, rst, &mut delay, delay_us).unwrap();
//     eprintln!("{}", 4);
//     let mut display = Display7in5::default();
//
//     // Build the style
//     eprintln!("{}", 5);
//     let style = MonoTextStyleBuilder::new()
//         .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
//         .text_color(Color::White)
//         .background_color(Color::Black)
//         .build();
//     eprintln!("{}", 6);
//     let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
//
//     // Draw some text at a certain point using the specified text style
//     let _ = Text::with_text_style("It's working-WoB!", Point::new(5, 8), style, text_style)
//         .draw(&mut display);
//
//     eprintln!("{}", 7);
//     epd.sleep(spi, &mut delay).unwrap();
// }
