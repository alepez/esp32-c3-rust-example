use std::time::Duration;

use esp_idf_sys as _;

use m5stamp_c3_example::application::App;
use m5stamp_c3_example::config::Config;
use m5stamp_c3_example::platform::Platform;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let config = Config::new();

    log::info!("Create platform");
    let platform = Platform::new(&config);

    log::info!("Create app");
    let mut app = App::new(&platform);

    let period = Duration::from_millis(20);

    log::info!("Start loop");
    loop {
        std::thread::sleep(period);
        app.update();
    }
}
