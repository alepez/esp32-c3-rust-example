use log::*;

pub struct Platform<'a> {
    pub sys_time: &'a dyn SystemTime,
    pub led: &'a mut dyn Led,
}

pub struct App<'a> {
    platform: Platform<'a>,
}

impl<'a> App<'a> {
    pub fn new(platform: Platform<'a>) -> Self {
        Self { platform }
    }

    pub fn update(&mut self) {
        let now = self.platform.sys_time.now().as_secs();
        info!("sys_time: {}", now);
        let color = Color {
            r: now as u8,
            g: now as u8,
            b: now as u8,
        };

        self.platform.led.set_color(color);
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