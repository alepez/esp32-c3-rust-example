use crate::rbg_led::RgbLed;

mod rbg_led;

pub struct Platform {
    pub led: RgbLed,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            led: RgbLed::new(),
        }
    }
}

pub struct RgbLedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}