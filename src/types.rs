use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
    pub base_url: String,
}

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("api responsed with bad request (400)")]
    BadRequest400,
    #[error("api responsed with unauthorized (401)")]
    Unauthorized401,
    #[error("api responsed with non acceptable (406)")]
    NotAcceptable406,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {}

#[derive(Deserialize, Debug, Clone)]
pub enum View {
    Global,
    Personal,
    User(u32),
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                View::Global => "global",
                View::Personal => "personal",
                View::User(_) => "user",
            }
        )
    }
}
