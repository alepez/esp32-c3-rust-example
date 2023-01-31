use crate::config::Config;
use crate::drivers::rgb_led::RgbLed;
use crate::drivers::wifi::Wifi;

pub struct Platform {
    pub wifi: Wifi,
    pub rgb_led: RgbLed,
}

impl Platform {
    pub fn new(config: &Config) -> Self {
        let wifi = Wifi::new(&config.wifi).expect("Cannot setup wifi");
        let rgb_led = RgbLed::new();
        Self { wifi, rgb_led }
    }
}
