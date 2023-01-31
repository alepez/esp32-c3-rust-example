use std::cell::RefCell;

use crate::WifiConfig;

mod esp_wifi;

#[derive(Default)]
pub struct Wifi(RefCell<Option<esp_wifi::Wifi<'static>>>);

impl Wifi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&self, config: &WifiConfig) {
        *self.0.borrow_mut() = esp_wifi::wifi(&config.ssid, &config.psk).ok();
    }
}

