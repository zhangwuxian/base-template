use serde::{Deserialize, Serialize};

use super::{load_config, ConfigTrait};

/// Settings for application
/// unmodifiable, set via config file when app start
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub log: LogConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    pub log_config: String,
    pub log_path: String,
}

pub const ENV_PREFIX: &str = "SAMPLE_SETTINGS_";

impl ConfigTrait for Settings {
    fn new(conf_path: &str) -> Self {
        load_config::<Settings>(ENV_PREFIX, conf_path, "config/settings.toml").unwrap_or_else(
            |_| {
                panic!(
                    "App settings initialize failed, check the settings file: `config/settings.toml`"
                );
            },
        )
    }
}
