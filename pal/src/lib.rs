pub struct Platform {
    pub sys_time: Box<dyn SystemTime>,
    pub led: Box<dyn Led>,
}

pub trait SystemTime {
    fn now(&self) -> std::time::Duration;
}

pub struct LedColor(rgb::RGB8);

impl Into<rgb::RGB8> for LedColor {
    fn into(self) -> rgb::RGB8 {
        self.0
    }
}

impl From<colors_transform::Rgb> for LedColor {
    fn from(x: colors_transform::Rgb) -> Self {
        use colors_transform::Color;
        LedColor(rgb::RGB8 {
            r: x.get_red() as u8,
            g: x.get_green() as u8,
            b: x.get_blue() as u8,
        })
    }
}

pub trait Led {
    fn set_color(&self, color: LedColor);
}