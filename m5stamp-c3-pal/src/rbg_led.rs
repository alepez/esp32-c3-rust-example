mod ws2812;

use std::cell::RefCell;
use ws2812::WS2812RMT;

pub struct RgbLed(RefCell<WS2812RMT>);

impl RgbLed {
    pub fn new() -> Self {
        Self(RefCell::new(WS2812RMT::new().unwrap()))
    }
}

impl pal::RgbLed for RgbLed {
    fn set_color(&self, color: pal::RgbLedColor) {
        self.0.borrow_mut().set_color(color.r, color.g, color.b).ok();
    }
}