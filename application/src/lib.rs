use std::time::Duration;

pub struct Platform {
    pub sys_time: Box<dyn SystemTime>,
    pub led: Box<dyn Led>,
}

#[allow(dead_code)]
pub struct App<'a> {
    platform: &'a Platform,
    led_controller: LedController<'a>,
}

impl<'a> App<'a> {
    pub fn new(platform: &'a Platform) -> Self {
        let led_controller = LedController { platform };
        let app = Self {
            platform,
            led_controller,
        };

        app
    }

    pub fn update(&mut self) {
        self.led_controller.update();
    }
}

pub trait SystemTime {
    fn now(&self) -> Duration;
}

pub struct Color(rgb::RGB8);

pub trait Led {
    fn set_color(&self, color: Color);
}

struct LedController<'a> {
    platform: &'a Platform,
}

impl<'a> LedController<'a> {
    pub fn update(&mut self) {
        let now = self.platform.sys_time.now();
        let hue = time_to_hue(now, Duration::from_secs(10));
        let color = huw_to_color(hue);
        self.platform.led.set_color(color);
    }
}

fn time_to_hue(time: Duration, period: Duration) -> f32 {
    let time = time.as_millis() as f32;
    let period = period.as_millis() as f32;
    let normalized = (time % period) / period;
    normalized * 360.0
}

fn huw_to_color(hue: f32) -> Color {
    use colors_transform::Color;
    let hsl = colors_transform::Hsl::from(hue as f32, 100.0, 50.0);
    let rgb = hsl.to_rgb();
    crate::Color(rgb::RGB8 {
        r: rgb.get_red() as u8,
        g: rgb.get_green() as u8,
        b: rgb.get_blue() as u8,
    })
}

impl Into<rgb::RGB8> for Color {
    fn into(self) -> rgb::RGB8 {
        self.0
    }
}