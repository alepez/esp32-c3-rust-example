pub trait Platform {
    fn sys_time(&self) -> &dyn SystemTime;
    fn led(&self) -> &dyn RgbLed;
    fn wifi(&self) -> &dyn Wifi;
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

pub struct WifiConfig {
    pub ssid: String,
    pub psk: String,
}

pub trait Wifi {
    fn setup(&self, config: &WifiConfig);
}
