use std::cell::RefCell;

mod esp_wifi;

#[derive(Default)]
pub struct Wifi(RefCell<Option<esp_wifi::Wifi<'static>>>);

impl Wifi {
    pub fn new() -> Self {
        Self::default()
    }
}

impl pal::Wifi for Wifi {
    fn setup(&self, config: &pal::WifiConfig) {
        *self.0.borrow_mut() = esp_wifi::wifi(&config.ssid, &config.psk).ok();
    }
}

