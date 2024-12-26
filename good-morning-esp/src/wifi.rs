use std::{
    net::Ipv4Addr,
    thread::sleep,
    time::{Duration, Instant},
};

use dotenvy_macro::dotenv;
use embedded_svc::wifi::{ClientConfiguration as WifiClientConfiguration, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    nvs::{EspNvsPartition, NvsDefault},
    wifi::EspWifi,
};
use good_morning_lib::BadMorning;

pub struct WifiConnection<'a> {
    wifi_driver: EspWifi<'a>,
}

impl Drop for WifiConnection<'_> {
    fn drop(&mut self) {
        match self.wifi_driver.disconnect() {
            Err(_) => println!("Failed to disconnect wifi!"),
            _ => (),
        }
    }
}

pub fn connect_to_wifi_with_timeout(
    timeout: Duration,
    modem: Modem,
    sys_loop: EspEventLoop<System>,
    nvs: EspNvsPartition<NvsDefault>,
) -> Result<WifiConnection<'static>, BadMorning> {
    let mut wifi_driver = EspWifi::new(modem, sys_loop, Some(nvs)).unwrap();
    let ssid = dotenv!("WIFI_SSID");
    let password = dotenv!("WIFI_PASS");
    wifi_driver
        .set_configuration(&Configuration::Client(WifiClientConfiguration {
            ssid: ssid.try_into().unwrap(),
            password: password.try_into().unwrap(),
            ..Default::default()
        }))
        .unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    let task_start = Instant::now();
    sleep(Duration::from_millis(1500));
    while !wifi_driver.is_connected().unwrap() {
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
        if task_start.elapsed() > timeout {
            return Err(BadMorning::WifiTimeout);
        }
        sleep(Duration::from_millis(250));
    }
    println!("Connected to wifi!");
    sleep(Duration::from_millis(500));

    loop {
        let ip_info = wifi_driver.sta_netif().get_ip_info().unwrap();
        if ip_info.ip != Ipv4Addr::new(0, 0, 0, 0) {
            println!("Got IP!");
            break;
        }
        if task_start.elapsed() > timeout {
            return Err(BadMorning::IpTimeout);
        }
        sleep(Duration::from_millis(150));
    }

    Ok(WifiConnection { wifi_driver })
}
