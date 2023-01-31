use std::time::Duration;

use esp_idf_sys as _;
use log::*;

use m5stamp_c3_example::application::App;
use m5stamp_c3_example::drivers::rgb_led::RgbLed;
use m5stamp_c3_example::drivers::wifi::{Wifi, WifiConfig};

#[toml_cfg::toml_config]
pub struct Config {
    #[default(WifiConfig::default())]
    wifi: WifiConfig<'static>,
}

fn main() -> anyhow::Result<()> {
    let app_config = CONFIG;

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting");

    let _wifi = Wifi::new(app_config.wifi).expect("Cannot setup wifi");

    let led = RgbLed::new();

    let mut app = App::new(&led);
    let period = Duration::from_millis(20);

    loop {
        std::thread::sleep(period);
        app.update();
    }
}
