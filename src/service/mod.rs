use thiserror::Error;

pub mod config;
pub mod login;

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("api responsed with bad request (400)")]
    BadRequest400,
    #[error("api responsed with unauthorized (401)")]
    Unauthorized401,
}

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("config fetch failed")]
    FetchFailed,
}
