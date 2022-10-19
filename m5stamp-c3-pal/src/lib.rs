use std::cell::RefCell;
use std::time::Duration;

struct SystemTime(esp_idf_svc::systime::EspSystemTime);

impl pal::SystemTime for SystemTime {
    fn now(&self) -> Duration {
        use embedded_svc::sys_time::SystemTime;
        self.0.now()
    }
}

struct RgbLed(RefCell<m5stamp_c3_bsc::led::WS2812RMT>);

impl pal::RgbLed for RgbLed {
    fn set_color(&self, color: pal::RgbLedColor) {
        let color = rgb::RGB8 {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        self.0.borrow_mut().set_pixel(color).ok();
    }
}

pub struct Platform {
    sys_time: SystemTime,
    led: RgbLed,
}

impl Platform {
    pub fn new() -> Self {
        let led = m5stamp_c3_bsc::led::WS2812RMT::new().unwrap();

        Self {
            sys_time: SystemTime(esp_idf_svc::systime::EspSystemTime),
            led: RgbLed(RefCell::new(led)),
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