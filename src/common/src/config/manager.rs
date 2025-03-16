use std::sync::OnceLock;

use tokio::sync::RwLock;

use super::{
    configs::AppConfig,
    settings::Settings,
    watcher::{ConfigWatcher, ConfigWatcherType},
    ConfigTrait,
};

#[derive(Debug)]
pub struct ConfigManager {
    pub settings: Settings,
    pub configs: RwLock<AppConfig>,
    pub watchers: Vec<ConfigWatcher>,
}

impl ConfigManager {
    pub fn new(settings_path: &str, configs_path: &str, watchers: Vec<ConfigWatcher>) -> Self {
        Self {
            settings: Settings::new(settings_path),
            configs: RwLock::new(AppConfig::new(configs_path)),
            watchers,
        }
    }

    pub fn get_app_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn get_app_configs(&self) -> &RwLock<AppConfig> {
        &self.configs
    }

    pub async fn start_watching(&self) {
        for watcher in &self.watchers {
            watcher.start_watching().await;
        }
    }

    pub async fn stop_watching(&self) {
        for watcher in &self.watchers {
            watcher.stop_watching().await;
        }
    }
}

static APP_CONFIG_MANAGER: OnceLock<ConfigManager> = OnceLock::new();

pub async fn init_app_config_manager(settings_path: &str, configs_path: &str) {
    // watchers
    let watchers = vec![ConfigWatcher::new(
        configs_path,
        ConfigWatcherType::AppConfigs,
    )];

    let config_manager = APP_CONFIG_MANAGER
        .get_or_init(|| ConfigManager::new(settings_path, configs_path, watchers));

    // start watching
    config_manager.start_watching().await;
}

pub fn get_app_config_manager() -> Option<&'static ConfigManager> {
    APP_CONFIG_MANAGER.get()
}

pub fn get_app_settings() -> &'static Settings {
    match APP_CONFIG_MANAGER.get() {
        Some(settings) => settings.get_app_settings(),
        None => {
            panic!(
                "get_app_settings failed: App settings is not initialized, check the settings file: `config/settings.toml`"
            );
        }
    }
}

pub fn get_app_conf() -> &'static RwLock<AppConfig> {
    match APP_CONFIG_MANAGER.get() {
        Some(config) => config.get_app_configs(),
        None => {
            panic!("get_app_conf failed: App configuration is not initialized, check the configuration file: `config/configs.toml`");
        }
    }
}

pub async fn stop_conf_watching_when_graceful_shutdown() {
    match APP_CONFIG_MANAGER.get() {
        Some(config) => config.stop_watching().await,
        None => {
            panic!("stop_conf_watching_when_graceful_shutdown failed: App configuration is not initialized, check the configuration file: `config/configs.toml`");
        }
    }
}
