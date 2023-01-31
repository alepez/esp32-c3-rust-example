use std::time::Duration;

use m5stamp_c3_pal::{RgbLed, RgbLedColor};

#[allow(dead_code)]
pub struct App<'a> {
    led_controller: LedController<'a>,
}

impl<'a> App<'a> {
    pub fn new(led: &'a RgbLed) -> Self {
        let led_controller = LedController { led };
        let app = Self {
            led_controller,
        };

        app
    }

    pub fn update(&mut self) {
        self.led_controller.update();
    }
}

struct LedController<'a> {
    led: &'a RgbLed,
}

impl<'a> LedController<'a> {
    pub fn update(&mut self) {
        let now = esp_idf_svc::systime::EspSystemTime.now();
        let hue = time_to_hue(now, Duration::from_secs(10));
        let color = huw_to_color(hue);
        self.led.set_color(color);
    }
}

fn time_to_hue(time: Duration, period: Duration) -> f32 {
    let time = time.as_millis() as f32;
    let period = period.as_millis() as f32;
    let normalized = (time % period) / period;
    normalized * 360.0
}

fn huw_to_color(hue: f32) -> RgbLedColor {
    use colors_transform::Color;
    let hsl = colors_transform::Hsl::from(hue as f32, 100.0, 50.0);
    let rgb = hsl.to_rgb();
    RgbLedColor {
        r: rgb.get_red() as u8,
        g: rgb.get_green() as u8,
        b: rgb.get_blue() as u8,
    }
}