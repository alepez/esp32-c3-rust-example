use log::*;

pub struct Platform {
    pub sys_time: Box<dyn SystemTime>,
    pub led: Box<dyn Led>,
}

pub struct App {
    platform: Platform,
}

impl App {
    pub fn new(platform: Platform) -> Self {
        Self { platform }
    }

    pub fn update(&mut self) {
        let now = self.platform.sys_time.now().as_secs();
        info!("sys_time: {}", now);
        let mut led_controller = LedController { led: self.platform.led.as_mut() };
        led_controller.update(self.platform.sys_time.as_ref());
    }
}

pub trait SystemTime {
    fn now(&self) -> core::time::Duration;
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait Led {
    fn set_color(&mut self, color: Color);
}

struct LedController<'a> {
    led: &'a mut dyn Led,
}

impl<'a> LedController<'a> {
    pub fn update(&mut self, sys_time: &dyn SystemTime) {
        use colors_transform::Color;

        let now = sys_time.now().as_secs();
        let hue = now % 360;
        let hsl = colors_transform::Hsl::from(hue as f32, 100.0, 50.0);
        let rgb = hsl.to_rgb();
        let color = crate::Color {
            r: rgb.get_red() as u8,
            g: rgb.get_green() as u8,
            b: rgb.get_blue() as u8,
        };
        self.led.set_color(color);
    }
}