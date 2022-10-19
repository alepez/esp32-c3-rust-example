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
        let now = sys_time.now().as_secs();
        let color = Color {
            r: now as u8,
            g: now as u8,
            b: now as u8,
        };
        self.led.set_color(color);
    }
}