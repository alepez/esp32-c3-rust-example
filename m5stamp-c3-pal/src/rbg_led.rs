use std::cell::RefCell;

pub struct RgbLed(RefCell<m5stamp_c3_bsc::led::WS2812RMT>);

impl RgbLed {
    pub fn new() -> Self {
        Self(RefCell::new(m5stamp_c3_bsc::led::WS2812RMT::new().unwrap()))
    }
}

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