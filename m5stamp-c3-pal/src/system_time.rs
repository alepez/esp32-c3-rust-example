use std::time::Duration;

pub struct SystemTime(esp_idf_svc::systime::EspSystemTime);

impl SystemTime {
    pub fn new() -> Self {
        Self(esp_idf_svc::systime::EspSystemTime)
    }
}

impl pal::SystemTime for SystemTime {
    fn now(&self) -> Duration {
        self.0.now()
    }
}

