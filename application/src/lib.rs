use log::*;

pub struct App<'a> {
    sys_time: &'a dyn SystemTime,
}

impl<'a> App<'a> {
    pub fn update(&mut self) {
        info!("sys_time: {}", self.sys_time.now().as_secs());
    }
}

#[derive(Default)]
pub struct AppBuilder<'a> {
    sys_time: Option<&'a dyn SystemTime>,
}

impl<'a> AppBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> App<'a> {
        App { sys_time: self.sys_time.unwrap() }
    }

    pub fn with_sys_time(mut self, sys_time: &'a dyn SystemTime) -> Self {
        self.sys_time = Some(sys_time);
        self
    }
}

pub trait SystemTime {
    fn now(&self) -> core::time::Duration;
}