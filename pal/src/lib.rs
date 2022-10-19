pub struct Platform {
    pub sys_time: Box<dyn SystemTime>,
    pub led: Box<dyn RgbLed>,
}

pub trait SystemTime {
    fn now(&self) -> std::time::Duration;
}

pub struct RgbLedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait RgbLed {
    fn set_color(&self, color: RgbLedColor);
}