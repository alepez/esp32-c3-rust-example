use crate::drivers::wifi::WifiConfig;

#[toml_cfg::toml_config]
pub struct Config {
    #[default(WifiConfig::default())]
    pub wifi: WifiConfig<'static>,
}

impl Config {
    pub fn new() -> Self {
        CONFIG
    }
}
