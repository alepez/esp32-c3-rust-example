mod rbg_led;
mod system_time;

use crate::system_time::SystemTime;
use crate::rbg_led::RgbLed;

pub struct Platform {
    sys_time: SystemTime,
    led: RgbLed,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            sys_time: SystemTime::new(),
            led: RgbLed::new(),
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
}