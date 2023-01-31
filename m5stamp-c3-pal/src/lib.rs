use crate::rbg_led::RgbLed;
use crate::system_time::SystemTime;

mod rbg_led;
mod system_time;

pub struct Platform {
    pub sys_time: SystemTime,
    pub led: RgbLed,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            sys_time: SystemTime::new(),
            led: RgbLed::new(),
        }
    }
}

pub struct RgbLedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}