use std::time::Duration;
use esp_idf_sys as _;
use log::*;
use pal::Platform;
use pal::WifiConfig;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    let app_config = CONFIG;

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting");

    let platform = m5stamp_c3_pal::Platform::new();

    platform.wifi().setup(&WifiConfig {
        ssid: app_config.wifi_ssid.to_string(),
        psk: app_config.wifi_psk.to_string(),
    });

    let mut app = application::App::new(&platform);
    let period = Duration::from_millis(20);

    loop {
        std::thread::sleep(period);
        app.update();
    }
}