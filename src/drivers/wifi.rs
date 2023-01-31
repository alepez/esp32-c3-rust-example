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

pub struct Wifi {
    _esp_wifi: EspWifi<'static>,
}

pub struct WifiConfig<'a> {
    pub(crate) ssid: &'a str,
    pub(crate) password: &'a str,
}

impl WifiConfig<'_> {
    pub const fn default() -> Self {
        Self {
            ssid: "",
            password: "",
        }
    }
}

impl TryInto<Configuration> for &WifiConfig<'_> {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Configuration> {
        let &WifiConfig { ssid, password } = self;

        let mut auth_method = AuthMethod::WPA2Personal;

        if ssid.is_empty() {
            bail!("missing WiFi name")
        }

        if password.is_empty() {
            auth_method = AuthMethod::None;
            log::info!("Wifi password is empty");
        }

        let config = ClientConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            channel: Default::default(),
            auth_method,
            ..Default::default()
        };

        Ok(Configuration::Client(config))
    }
}

impl Wifi {
    pub fn new(config: &WifiConfig) -> anyhow::Result<Wifi> {
        let peripherals = Peripherals::take().unwrap();
        let sys_loop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();

        let mut wifi = EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;
        let config = config.try_into()?;
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

        let wifi = Wifi { _esp_wifi: wifi };

        Ok(wifi)
    }
}
