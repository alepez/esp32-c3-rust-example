use std::cell::RefCell;

use ws2812::WS2812RMT;

use crate::RgbLedColor;

mod ws2812;

pub struct RgbLed(RefCell<WS2812RMT>);

impl RgbLed {
    pub fn new() -> Self {
        Self(RefCell::new(WS2812RMT::new().unwrap()))
    }

    pub fn set_color(&self, color: RgbLedColor) {
        self.0.borrow_mut().set_color(color.r, color.g, color.b).ok();
    }
}