use crate::config::get_app_conf;
use crate::tools::{create_fold, file_exists, read_file};

pub fn init_app_log() {
    // get app conf
    let conf = get_app_conf();

    // check log config.yaml
    if !file_exists(&conf.log.log_config) {
        panic!(
            "Logging configuration file {} does not exist",
            conf.log.log_config
        );
    }

    // try to create log path
    match create_fold(&conf.log.log_path) {
        Ok(()) => {}
        Err(e) => {
            panic!(
                "Failed to initialize log directory {}, error: {e:?}",
                conf.log.log_path
            );
        }
    }

    // read log config file
    let content = match read_file(&conf.log.log_config) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    // replace log path
    let config_content = content.replace("{$path}", &conf.log.log_path);

    // parse log config.yaml
    let config = match serde_yaml::from_str(&config_content) {
        Ok(data) => data,
        Err(e) => {
            panic!(
                "Failed to parse the contents of the config file {} with error message: {e}",
                conf.log.log_config,
            );
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
