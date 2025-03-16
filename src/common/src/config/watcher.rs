use std::{
    sync::{mpsc::Sender, Arc},
    time::Duration,
};

use log::{error, info};
use notify::{
    event::{DataChange, ModifyKind},
    Config as NotifyConfig, Error, Event, RecommendedWatcher, Watcher,
};
use tokio::sync::Mutex;

use crate::config::{configs::AppConfig, manager::get_app_config_manager, ConfigTrait};

const CTL_C_TO_CLOSE_EVENT_INFO: &str = "Ctl_C_To_Close";

type CloseSender = Arc<Mutex<Option<Sender<Result<Event, Error>>>>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigWatcherType {
    AppConfigs,
}

#[derive(Debug)]
pub struct ConfigWatcher {
    pub conf_path: String,
    pub w_type: ConfigWatcherType,
    pub tx: CloseSender,
}

impl ConfigWatcher {
    pub fn new(conf_path: &str, w_type: ConfigWatcherType) -> Self {
        Self {
            conf_path: conf_path.to_string(),
            w_type,
            tx: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_watching(&self) {
        let (tx, rx) = std::sync::mpsc::channel();
        *self.tx.lock().await = Some(tx.clone());

        let mut watcher: RecommendedWatcher = match Watcher::new(
            tx,
            NotifyConfig::default().with_poll_interval(Duration::from_secs(10)),
        ) {
            Ok(w) => w,
            Err(e) => {
                panic!("watch config file error: {e}");
            }
        };

        watcher
            .watch(self.conf_path.as_ref(), notify::RecursiveMode::NonRecursive)
            .unwrap();

        let conf_path = self.conf_path.clone();
        let w_type = self.w_type.clone();
        tokio::spawn(async move {
            loop {
                match rx.recv() {
                    Ok(Ok(Event {
                        kind:
                            notify::event::EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                        ..
                    })) => {
                        if w_type == ConfigWatcherType::AppConfigs {
                            match get_app_config_manager() {
                                Some(config) => {
                                    *config.get_app_configs().write().await =
                                        AppConfig::new(&conf_path);
                                    info!("refresh_app_configs success.");
                                }
                                None => {
                                    panic!("refresh_app_configs failed: App configuration is not initialized, check the configuration file: `{conf_path}`");
                                }
                            }
                        }
                    }
                    Ok(Ok(Event {
                        kind: notify::event::EventKind::Other,
                        attrs,
                        ..
                    })) => {
                        if attrs.info().is_some()
                            && attrs.info().unwrap() == CTL_C_TO_CLOSE_EVENT_INFO
                        {
                            break;
                        }
                    }
                    _ => {
                        error!("somethine wrong when watching config file: {conf_path}");
                        continue;
                    }
                }
            }

            if let Err(e) = watcher.unwatch(conf_path.as_ref()) {
                error!("Failed to unwatch config file: {e}");
            }
            info!("config hot loader is graceful shutdown.");
        });
    }

    pub async fn stop_watching(&self) {
        let close_event = Event::new(notify::EventKind::Other).set_info(CTL_C_TO_CLOSE_EVENT_INFO);
        match self.tx.lock().await.as_ref().unwrap().send(Ok(close_event)) {
            Ok(_) => {
                info!("Successful to send message to stop watching config files.");
            }
            Err(e) => {
                error!("Failed to send stop watching config file message, err: {e}");
            }
        }
    }
}
