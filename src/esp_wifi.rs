// based on https://github.com/ivmarkov/rust-esp32-std-demo/blob/main/src/main.rs
use std::net::Ipv4Addr;
use std::time::Duration;

use anyhow::bail;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::netif::{EspNetif, EspNetifWait};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{EspWifi, WifiWait};

#[allow(unused)]
pub struct Wifi<'a> {
    esp_wifi: EspWifi<'a>,
}

pub fn wifi(ssid: &str, psk: &str) -> anyhow::Result<Wifi<'static>> {
    let mut auth_method = AuthMethod::WPA2Personal;

    if ssid.is_empty() {
        bail!("missing WiFi name")
    }

    if psk.is_empty() {
        auth_method = AuthMethod::None;
        log::info!("Wifi password is empty");
    }

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;

    log::info!("Searching for Wifi network {}", ssid);

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = ours.map(|x| x.channel);

    log::info!("setting Wifi configuration");

    let config = ClientConfiguration {
        ssid: ssid.into(),
        password: psk.into(),
        channel,
        auth_method,
        ..Default::default()
    };

    let config = Configuration::Client(config);

    wifi.set_configuration(&config)?;

    wifi.start()?;

    let started = {
        let timeout = Duration::from_secs(20);
        let matcher = || wifi.is_started().unwrap();
        WifiWait::new(&sys_loop)?.wait_with_timeout(timeout, matcher)
    };

    if !started {
        bail!("Wifi did not start");
    }

    wifi.connect()?;

    let is_connected = {
        let timeout = Duration::from_secs(20);
        let matcher = || {
            wifi.driver().is_connected().unwrap()
                && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
        };
        EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sys_loop)?
            .wait_with_timeout(timeout, matcher)
    };

    if !is_connected {
        bail!("Wifi did not connect or did not receive a DHCP lease");
    }

    let ip_info = wifi.sta_netif().get_ip_info()?;

    log::info!("Wifi DHCP info: {:?}", ip_info);

    let wifi = Wifi { esp_wifi: wifi };

    Ok(wifi)
}
