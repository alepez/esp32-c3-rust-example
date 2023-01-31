mod rbg_led;
mod system_time;
mod wifi;

use crate::system_time::SystemTime;
use crate::rbg_led::RgbLed;
use crate::wifi::Wifi;

pub struct Platform {
    pub sys_time: SystemTime,
    pub led: RgbLed,
    pub wifi: Wifi,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            sys_time: SystemTime::new(),
            led: RgbLed::new(),
            wifi: Wifi::new(),
        }
    }
}

pub struct RgbLedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct WifiConfig {
    pub ssid: String,
    pub psk: String,
}
