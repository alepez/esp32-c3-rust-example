use m5stamp_c3_bsc as bsc;
use esp_idf_sys as _;
use log::*;
use m5stamp_c3_bsc::wifi::Wifi;
use embedded_svc::sys_time::SystemTime;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

const BLUE: bsc::led::RGB8 = bsc::led::RGB8::new(0, 0, 50);
const RED: bsc::led::RGB8 = bsc::led::RGB8::new(50, 0, 0);

fn setup_wifi(app_config: &Config) -> Option<Wifi> {
    bsc::wifi::wifi(app_config.wifi_ssid, app_config.wifi_psk).ok()
}

fn main() -> anyhow::Result<()> {
    let app_config = CONFIG;

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting");

    let mut led = bsc::led::WS2812RMT::new()?;

    let wifi = setup_wifi(&app_config);
    let has_wifi = wifi.is_some();
    let led_color = if has_wifi { BLUE } else { RED };
    led.set_pixel(led_color)?;

    let sys_time = esp_idf_svc::systime::EspSystemTime;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        info!("sys_time: {}", sys_time.now().as_secs());
    }
}
