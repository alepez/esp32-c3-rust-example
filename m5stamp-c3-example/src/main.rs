use std::time::Duration;
use esp_idf_sys as _;
use log::*;
use m5stamp_c3_bsc::wifi::Wifi;
use m5stamp_c3_pal::Platform;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn setup_wifi(app_config: &Config) -> Option<Wifi> {
    m5stamp_c3_bsc::wifi::wifi(app_config.wifi_ssid, app_config.wifi_psk).ok()
}

fn main() -> anyhow::Result<()> {
    let app_config = CONFIG;

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting");

    let _wifi = setup_wifi(&app_config);

    let platform = Platform::new();
    let mut app = application::App::new(&platform);
    let period = Duration::from_millis(20);

    loop {
        std::thread::sleep(period);
        app.update();
    }
}