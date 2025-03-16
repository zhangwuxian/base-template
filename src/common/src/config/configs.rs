use serde::{Deserialize, Serialize};

use super::{load_config, ConfigTrait};

/// Configurations for application
/// modifiable, hot loading when config file changed
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub app: App,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub app_name: String,
}


const ENV_PREFIX: &str = "SAMPLE_CONFIGS_";

impl ConfigTrait for AppConfig {
    fn new(conf_path: &str) -> Self {
        load_config::<AppConfig>(ENV_PREFIX, conf_path, "config/configs.toml").unwrap_or_else(|_| {
            panic!("App configuration initialize failed, check the configuration file: `config/configs.toml`");
        })
    }
}
