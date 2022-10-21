pub struct Wifi;

impl Wifi {
    pub fn new() -> Self {
        todo!()
    }
}

impl pal::Wifi for Wifi {
    fn setup(&self, _config: &pal::WifiConfig) {
        todo!()
    }
}