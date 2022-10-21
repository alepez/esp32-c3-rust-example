mod rbg_led;
mod system_time;
mod wifi;

use crate::system_time::SystemTime;
use crate::rbg_led::RgbLed;
use crate::wifi::Wifi;

pub struct Platform {
    sys_time: SystemTime,
    led: RgbLed,
    wifi: Wifi,
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

impl pal::Platform for Platform {
    fn sys_time(&self) -> &dyn pal::SystemTime {
        &self.sys_time
    }

    fn led(&self) -> &dyn pal::RgbLed {
        &self.led
    }

    fn wifi(&self) -> &dyn pal::Wifi { &self.wifi }
}