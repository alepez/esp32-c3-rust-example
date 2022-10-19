pub struct Platform {
    pub sys_time: Box<dyn SystemTime>,
    pub led: Box<dyn Led>,
}

pub trait SystemTime {
    fn now(&self) -> std::time::Duration;
}

pub struct LedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait Led {
    fn set_color(&self, color: LedColor);
}