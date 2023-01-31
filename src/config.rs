use crate::drivers::wifi::WifiConfig;

#[toml_cfg::toml_config]
pub struct TomlConfig {
    #[default("")]
    pub wifi_ssid: &'static str,
    #[default("")]
    pub wifi_password: &'static str,
}

pub struct Config {
    pub wifi: WifiConfig<'static>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            wifi: WifiConfig {
                ssid: TOML_CONFIG.wifi_ssid,
                password: TOML_CONFIG.wifi_password,
            },
        }
    }
}
