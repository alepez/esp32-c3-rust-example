use log::*;

pub struct App<'a> {
    sys_time: &'a dyn SystemTime,
    led: &'a mut dyn Led,
}

impl<'a> App<'a> {
    pub fn update(&mut self) {
        let now = self.sys_time.now().as_secs();
        info!("sys_time: {}", now);
        let color = Color {
            r: now as u8,
            g: now as u8,
            b: now as u8,
        };

        self.led.set_color(color);
    }
}

#[derive(Default)]
pub struct AppBuilder<'a> {
    sys_time: Option<&'a dyn SystemTime>,
    led: Option<&'a mut dyn Led>,
}

impl<'a> AppBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Option<App<'a>> {
        let app = App {
            sys_time: self.sys_time?,
            led: self.led?,
        };

        Some(app)
    }

    pub fn with_sys_time(mut self, sys_time: &'a dyn SystemTime) -> Self {
        self.sys_time = Some(sys_time);
        self
    }

    pub fn with_led(mut self, led: &'a mut dyn Led) -> Self {
        self.led = Some(led);
        self
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