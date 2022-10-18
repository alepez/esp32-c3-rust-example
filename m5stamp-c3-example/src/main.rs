use m5stamp_c3_bsc as bsc;
use esp_idf_sys as _;
use log::*;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

const GREEN: bsc::led::RGB8 = bsc::led::RGB8::new(0, 50, 0);
const BLUE: bsc::led::RGB8 = bsc::led::RGB8::new(0, 0, 50);
const RED: bsc::led::RGB8 = bsc::led::RGB8::new(50, 0, 0);
const YELLOW: bsc::led::RGB8 = bsc::led::RGB8::new(50, 50, 0);

fn main() -> anyhow::Result<()> {
    let app_config = CONFIG;

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting");

    let mut led = bsc::led::WS2812RMT::new()?;
    led.set_pixel(YELLOW)?;

    let _wifi = match bsc::wifi::wifi(app_config.wifi_ssid, app_config.wifi_psk) {
        Ok(inner) => inner,
        Err(err) => {
            led.set_pixel(RED)?;
            anyhow::bail!("could not connect to Wi-Fi network: {:?}", err)
        }
    };

    loop {
        led.set_pixel(BLUE)?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        info!("Loop");
        led.set_pixel(GREEN)?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
