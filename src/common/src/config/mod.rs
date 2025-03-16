use config::{Config, Environment, File};
use serde::de::DeserializeOwned;

use std::path::Path;

use crate::errors::CommonError;

pub mod configs;
pub mod manager;
pub mod settings;
pub mod watcher;

trait ConfigTrait {
    fn new(conf_path: &str) -> Self;

    // fn validate(&self) -> Result<(), String>;
}

fn load_config<T>(env_prefix: &str, conf_path: &str, default_file: &str) -> Result<T, CommonError>
where
    T: ConfigTrait + DeserializeOwned,
{
    let mut builder = Config::builder();

    // config priority: ENV > specify config file > default config file
    builder = builder.add_source(File::with_name(default_file).required(false));
    if Path::new(conf_path).exists() {
        builder = builder.add_source(File::with_name(conf_path));
    }
    builder = builder.add_source(Environment::with_prefix(env_prefix).separator("__"));

    let config = builder.build().map_err(|e| {
        CommonError::SystemError(format!(
            "Error building configuration: {conf_path}, err: {e}"
        ))
    })?;

    match config.try_deserialize() {
        Ok(settings) => Ok(settings),
        Err(e) => Err(CommonError::SystemError(format!(
            "Error deserializing configuration: {conf_path}, err: {e:#?}"
        ))),
    }

    // TODO: validate settings
    // settings.validate()?;
}

#[cfg(test)]
mod tests {

    use super::manager::init_app_config_manager;
    use crate::config::manager::{
        get_app_conf, get_app_settings, stop_conf_watching_when_graceful_shutdown,
    };

    #[tokio::test]
    async fn test_config_manager() {
        let settings_path = format!("{}/../../config/settings.toml", env!("CARGO_MANIFEST_DIR"));
        println!("settings_path: {}", settings_path);

        let config_path = format!("{}/../../config/configs.toml", env!("CARGO_MANIFEST_DIR"));
        println!("config_path: {}", config_path);

        init_app_config_manager(&settings_path, &config_path).await;

        let settings = get_app_settings();
        assert_eq!(settings.log.log_config, "./config/log4rs.yaml");
        assert_eq!(settings.log.log_path, "./logs");

        let config = get_app_conf().read().await;
        assert_eq!(config.app.app_name, "sample");

        stop_conf_watching_when_graceful_shutdown().await;
    }
}
