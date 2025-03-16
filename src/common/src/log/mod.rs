use crate::config::manager::get_app_settings;
use crate::tools::{create_fold, file_exists, read_file};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static APP_ABSOLUTE_PATH: OnceLock<Option<String>> = OnceLock::new();

pub fn init_app_log() {
    // get app conf
    let settings = get_app_settings();

    // unified path
    let log_config = handle_relative_path(&settings.log.log_config);
    let log_path = handle_relative_path(&settings.log.log_path);

    // check log config.yaml
    if !file_exists(&log_config) {
        panic!("Logging configuration file {log_config} does not exist");
    }

    // try to create log path
    match create_fold(&log_path) {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to initialize log directory {log_path}, error: {e:#?}");
        }
    }

    // read log config file
    let content = match read_file(&log_config) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    // replace log path
    let config_content = content.replace("{$path}", &log_path);

    // parse log config.yaml
    let config = match serde_yaml::from_str(&config_content) {
        Ok(data) => data,
        Err(e) => {
            panic!("Failed to parse the contents of the config file {log_config} with error message: {e}");
        }
    };

    // init log config
    match log4rs::init_raw_config(config) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }
}

fn handle_relative_path(conf: &str) -> String {
    let is_relative = Path::new(&conf).is_relative();
    if !is_relative {
        return conf.to_string();
    }

    let absolute_path_prefix = APP_ABSOLUTE_PATH.get_or_init(|| {
        if let Ok(current_dir) = env::current_dir() {
            if cfg!(debug_assertions) {
                Some(current_dir.to_str().unwrap().to_string())
            } else {
                current_dir.join("../").to_str().map(|s| s.to_string())
            }
        } else {
            None
        }
    });

    match absolute_path_prefix {
        Some(prefix) => {
            let path = PathBuf::from(prefix.to_owned());
            path.join(conf).to_str().unwrap().to_string()
        }
        None => conf.to_string(),
    }
}
