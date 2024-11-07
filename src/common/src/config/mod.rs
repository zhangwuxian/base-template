use crate::tools::read_file;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,
}

#[derive(Serialize, Deserialize)]
pub struct LogConfig {
    pub log_config: String,
    pub log_path: String,
}

static APP_CONF: OnceLock<AppConfig> = OnceLock::new();

pub fn init_app_conf_by_path(config_path: &str) -> &'static AppConfig {
    // n.b. static items do not call [`Drop`] on program termination, so if
    // [`DeepThought`] impls Drop, that will not be used for this instance.
    APP_CONF.get_or_init(|| {
        let content = match read_file(config_path) {
            Ok(content) => content,
            Err(e) => panic!("Error reading configuration file: {config_path}, err: {e}"),
        };
        let pc_config: AppConfig = match toml::from_str(&content) {
            Ok(pc_config) => pc_config,
            Err(e) => panic!("Error parsing configuration file: {config_path}, err: {e}"),
        };
        pc_config
    })
}

pub fn get_app_conf() -> &'static AppConfig {
    match APP_CONF.get() {
        Some(config) => config,
        None => {
            panic!("App configuration is not initialized, check the configuration file.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_app_conf_by_path() {
        let config_path = format!("{}/../../config/sample.toml", env!("CARGO_MANIFEST_DIR"));
        println!("config_path: {}", config_path);
        let config = init_app_conf_by_path(&config_path);

        assert_eq!(config.log.log_config, "./config/log4rs.yaml");
        assert_eq!(config.log.log_path, "./logs")
    }
}
