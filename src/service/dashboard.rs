use anyhow::{anyhow, Result};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::*,
};

use crate::service::{ConfigService, LoginService};
use crate::types::{ApiError, StatusPaginate, View};

#[derive(Debug, Default)]
pub struct DashboardService;

impl DashboardService {
    pub fn statuses(
        view: View,
        page: u64,
        callback: Callback<Result<StatusPaginate>>,
    ) -> FetchTask {
        let config = ConfigService::get();
        let bearer = LoginService::get_format();
        let url = format!(
            "{}/api/v0/statuses?view={}&page={}",
            config.base_url, view, page
        );

        let request = Request::get(url)
            .header("Authorization", bearer)
            .header("Accept", "application/json")
            .body(Nothing)
            .unwrap();

        let handler = move |response: Response<Json<Result<StatusPaginate>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data);
            } else {
                if meta.status == 400 {
                    callback.emit(Err(anyhow!(ApiError::BadRequest400)));
                } else if meta.status == 401 {
                    callback.emit(Err(anyhow!(ApiError::Unauthorized401)));
                } else if meta.status == 406 {
                    callback.emit(Err(anyhow!(ApiError::NotAcceptable406)));
                }
            }
        };
        FetchService::fetch(request, handler.into()).unwrap()
    }
}
