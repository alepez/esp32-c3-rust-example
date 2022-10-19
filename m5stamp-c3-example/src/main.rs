use std::cell::RefCell;
use std::time::Duration;
use m5stamp_c3_bsc as bsc;
use esp_idf_sys as _;
use log::*;
use m5stamp_c3_bsc::wifi::Wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

const BLUE: bsc::led::RGB8 = bsc::led::RGB8::new(0, 0, 50);
const RED: bsc::led::RGB8 = bsc::led::RGB8::new(50, 0, 0);

fn setup_wifi(app_config: &Config) -> Option<Wifi> {
    bsc::wifi::wifi(app_config.wifi_ssid, app_config.wifi_psk).ok()
}

fn main() -> anyhow::Result<()> {
    let app_config = CONFIG;

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting");

    let mut led = bsc::led::WS2812RMT::new()?;

    let wifi = setup_wifi(&app_config);
    let has_wifi = wifi.is_some();
    let led_color = if has_wifi { BLUE } else { RED };
    led.set_pixel(led_color)?;

    let platform = Box::new(pal::Platform {
        sys_time: Box::new(SystemTime(esp_idf_svc::systime::EspSystemTime)),
        led: Box::new(Led(RefCell::new(led))),
    });

    let mut app = application::App::new(&platform);

    let period = Duration::from_millis(20);

    loop {
        std::thread::sleep(period);
        app.update();
    }
}

struct SystemTime(esp_idf_svc::systime::EspSystemTime);

impl pal::SystemTime for SystemTime {
    fn now(&self) -> Duration {
        use embedded_svc::sys_time::SystemTime;
        self.0.now()
    }
}

struct Led(RefCell<bsc::led::WS2812RMT>);

impl pal::Led for Led {
    fn set_color(&self, color: pal::LedColor) {
        let color = rgb::RGB8 {
            r: color.r,
            g: color.g,
            b: color.b,
        };
        self.0.borrow_mut().set_pixel(color).ok();
    }
}