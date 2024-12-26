#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WeatherCode {
    ClearSky = 0,                   // 0: Clear sky
    MainlyClear = 1,                // 1: Mainly clear
    PartlyCloudy = 2,               // 2: Partly cloudy
    Overcast = 3,                   // 3: Overcast
    Fog = 45,                       // 45: Fog
    DepositingRimeFog = 48,         // 48: Depositing rime fog
    LightDrizzle = 51,              // 51: Drizzle: Light intensity
    ModerateDrizzle = 53,           // 53: Drizzle: Moderate intensity
    DenseDrizzle = 55,              // 55: Drizzle: Dense intensity
    LightFreezingDrizzle = 56,      // 56: Freezing Drizzle: Light intensity
    DenseFreezingDrizzle = 57,      // 57: Freezing Drizzle: Dense intensity
    SlightRain = 61,                // 61: Rain: Slight intensity
    ModerateRain = 63,              // 63: Rain: Moderate intensity
    HeavyRain = 65,                 // 65: Rain: Heavy intensity
    LightFreezingRain = 66,         // 66: Freezing Rain: Light intensity
    HeavyFreezingRain = 67,         // 67: Freezing Rain: Heavy intensity
    SlightSnowfall = 71,            // 71: Snowfall: Slight intensity
    ModerateSnowfall = 73,          // 73: Snowfall: Moderate intensity
    HeavySnowfall = 75,             // 75: Snowfall: Heavy intensity
    SnowGrains = 77,                // 77: Snow grains
    SlightRainShowers = 80,         // 80: Rain showers: Slight intensity
    ModerateRainShowers = 81,       // 81: Rain showers: Moderate intensity
    ViolentRainShowers = 82,        // 82: Rain showers: Violent intensity
    SlightSnowShowers = 85,         // 85: Snow showers: Slight intensity
    HeavySnowShowers = 86,          // 86: Snow showers: Heavy intensity
    Thunderstorm = 95,              // 95: Thunderstorm: Slight or moderate
    ThunderstormWithHail = 96,      // 96: Thunderstorm with slight hail
    ThunderstormWithHeavyHail = 99, // 99: Thunderstorm with heavy hail
}

use serde::{de::Error, Deserialize, Serialize};
use WeatherCode::*;

impl WeatherCode {
    pub fn get_night_icon_big(&self) -> Option<&[u8]> {
        Some(include_bytes!(
            "../../resources/icons/weather-night.png.big.bmp"
        ))
    }
}

impl WeatherCode {
    pub fn get_icon_big(&self) -> &[u8] {
        match self {
            ClearSky => include_bytes!("../../resources/icons/sun.png.big.bmp"),
            MainlyClear => include_bytes!("../../resources/icons/sun.png.big.bmp"),
            PartlyCloudy => include_bytes!("../../resources/icons/cloudy.png.big.bmp"),
            Overcast => include_bytes!("../../resources/icons/cloudy2.png.big.bmp"),
            Fog => include_bytes!("../../resources/icons/fog.png.big.bmp"),
            SlightRain | ModerateRain | HeavyRain | SlightRainShowers => {
                include_bytes!("../../resources/icons/rainy-day.png.big.bmp")
            }
            SlightSnowfall | ModerateSnowfall | HeavySnowfall => {
                include_bytes!("../../resources/icons/rainy-day.png.big.bmp")
            }
            Thunderstorm | ThunderstormWithHail | ThunderstormWithHeavyHail => {
                include_bytes!("../../resources/icons/depression.png.big.bmp")
            }
            _ => include_bytes!("../../resources/icons/mixed.png.big.bmp"),
        }
    }
    pub fn get_icon_small(&self) -> &[u8] {
        match self {
            ClearSky => include_bytes!("../../resources/icons/sun.png.small.bmp"),
            MainlyClear => include_bytes!("../../resources/icons/sun.png.small.bmp"),
            PartlyCloudy => include_bytes!("../../resources/icons/cloudy.png.small.bmp"),
            Overcast => include_bytes!("../../resources/icons/cloudy2.png.small.bmp"),
            Fog => include_bytes!("../../resources/icons/fog.png.small.bmp"),
            SlightRain | ModerateRain | HeavyRain | SlightRainShowers => {
                include_bytes!("../../resources/icons/rainy-day.png.small.bmp")
            }
            SlightSnowfall | ModerateSnowfall | HeavySnowfall => {
                include_bytes!("../../resources/icons/rainy-day.png.small.bmp")
            }
            Thunderstorm | ThunderstormWithHail | ThunderstormWithHeavyHail => {
                include_bytes!("../../resources/icons/depression.png.small.bmp")
            }
            _ => include_bytes!("../../resources/icons/mixed.png.small.bmp"),
        }
    }
    pub fn get_icon_big_night(&self) -> Option<&[u8]> {
        match self {
            ClearSky => Some(include_bytes!("../../resources/icons/moon.png.big.bmp")),
            PartlyCloudy | MainlyClear => Some(include_bytes!(
                "../../resources/icons/moon-light.png.big.bmp"
            )),
            Overcast => Some(include_bytes!(
                "../../resources/icons/cloudy-night.png.big.bmp"
            )),
            _ => None,
        }
    }
    pub fn get_icon_small_night(&self) -> Option<&[u8]> {
        match self {
            ClearSky => Some(include_bytes!("../../resources/icons/moon.png.small.bmp")),
            PartlyCloudy | MainlyClear => Some(include_bytes!(
                "../../resources/icons/moon-light.png.small.bmp"
            )),
            Overcast => Some(include_bytes!(
                "../../resources/icons/cloudy-night.png.small.bmp"
            )),
            _ => None,
        }
    }
}

impl Serialize for WeatherCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for WeatherCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        // TODO Fix this
        match value {
            0 => Ok(ClearSky),                   // 0: Clear sky
            1 => Ok(MainlyClear),                // 1: Mainly clear
            2 => Ok(PartlyCloudy),               // 2: Partly cloudy
            3 => Ok(Overcast),                   // 3: Overcast
            45 => Ok(Fog),                       // 45: Fog
            48 => Ok(DepositingRimeFog),         // 48: Depositing rime fog
            51 => Ok(LightDrizzle),              // 51: Drizzle: Light intensity
            53 => Ok(ModerateDrizzle),           // 53: Drizzle: Moderate intensity
            55 => Ok(DenseDrizzle),              // 55: Drizzle: Dense intensity
            56 => Ok(LightFreezingDrizzle),      // 56: Freezing Drizzle: Light intensity
            57 => Ok(DenseFreezingDrizzle),      // 57: Freezing Drizzle: Dense intensity
            61 => Ok(SlightRain),                // 61: Rain: Slight intensity
            63 => Ok(ModerateRain),              // 63: Rain: Moderate intensity
            65 => Ok(HeavyRain),                 // 65: Rain: Heavy intensity
            66 => Ok(LightFreezingRain),         // 66: Freezing Rain: Light intensity
            67 => Ok(HeavyFreezingRain),         // 67: Freezing Rain: Heavy intensity
            71 => Ok(SlightSnowfall),            // 71: Snowfall: Slight intensity
            73 => Ok(ModerateSnowfall),          // 73: Snowfall: Moderate intensity
            75 => Ok(HeavySnowfall),             // 75: Snowfall: Heavy intensity
            77 => Ok(SnowGrains),                // 77: Snow grains
            80 => Ok(SlightRainShowers),         // 80: Rain showers: Slight intensity
            81 => Ok(ModerateRainShowers),       // 81: Rain showers: Moderate intensity
            82 => Ok(ViolentRainShowers),        // 82: Rain showers: Violent intensity
            85 => Ok(SlightSnowShowers),         // 85: Snow showers: Slight intensity
            86 => Ok(HeavySnowShowers),          // 86: Snow showers: Heavy intensity
            95 => Ok(Thunderstorm),              // 95: Thunderstorm: Slight or moderate
            96 => Ok(ThunderstormWithHail),      // 96: Thunderstorm with slight hail
            99 => Ok(ThunderstormWithHeavyHail), // 99: Thunderstorm with heavy hail
            x => Err(serde::de::Error::custom(format!(
                "Weather code {} not found",
                x
            ))),
        }
    }
}
