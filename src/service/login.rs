use crate::{service::config::ConfigService, types::ApiError};

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde_derive::*;
use serde_json::json;
use yew::{format::Json, prelude::*, services::fetch::*};

use std::sync::RwLock;

pub struct LoginService;

lazy_static! {
    static ref BEARER: RwLock<Option<String>> = RwLock::default();
}

impl LoginService {
    pub fn get() -> String {
        let bearer = BEARER.read().expect("lock poisoned");

        bearer.clone().expect("bearer none")
    }

    pub fn get_format() -> String {
        format!("Bearer {}", LoginService::get())
    }

    pub fn put(bearer: String) {
        let mut bearer_lock = BEARER.write().expect("lock poisoned");

        *bearer_lock = Some(bearer);
    }

    pub fn login(email: String, password: String, callback: Callback<Result<String>>) -> FetchTask {
        let config = ConfigService::get();
        let url = format!("{}/api/v0/auth/login", config.base_url);
        let handler = move |response: Response<Json<Result<BearerToken>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data.map(|bearer| bearer.token));
            } else {
                if meta.status == 400 {
                    callback.emit(Err(anyhow!(ApiError::BadRequest400)));
                } else if meta.status == 401 {
                    callback.emit(Err(anyhow!(ApiError::Unauthorized401)));
                }
            }
        };
        let body = json! ({ "email": email, "password": password });
        let request = Request::post(url)
            .header("content-type", "application/json")
            .body(Ok(body.to_string()))
            .unwrap();
        FetchService::fetch(request, handler.into()).unwrap()
    }
}

#[derive(Deserialize)]
struct BearerToken {
    token: String,
}
