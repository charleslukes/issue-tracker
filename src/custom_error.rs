use std::env::VarError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("AUTH_KEY: {0}")]
    AuthKeyError(#[from] VarError),
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
}
