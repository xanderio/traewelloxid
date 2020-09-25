use anyhow::{anyhow, Error};
use lazy_static::lazy_static;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::*,
};

use std::sync::RwLock;

use crate::Config;

lazy_static! {
    static ref CONFIG: RwLock<Option<Config>> = RwLock::default();
}

pub struct ConfigService;

impl ConfigService {
    /// get a copy of the global config
    /// Blocks till there is a config loaded
    pub fn get<'a>() -> Config {
        let config = CONFIG.read().expect("lock poisoned");

        if let None = *config {
            panic!("config none");
        }
        config.clone().unwrap()
    }

    /// Replace the global application configuration.
    /// This will block till a write lock can be acquired
    pub fn put(config: Config) {
        let mut config_lock = CONFIG.write().expect("lock poisoned");

        *config_lock = Some(config);
    }

    pub fn load(callback: Callback<Result<Config, Error>>) -> FetchTask {
        let url = "/config.json";
        let request = Request::get(url).body(Nothing).unwrap();

        let handler = move |response: Response<Json<Result<Config, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!("{}: error loading config", meta.status)))
            }
        };
        FetchService::fetch(request, handler.into()).unwrap()
    }
}
