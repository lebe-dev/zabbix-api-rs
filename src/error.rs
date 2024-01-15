use std::fmt::{Display, Formatter};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZabbixApiError {
    #[error("network error")]
    NetworkError(#[from] reqwest::Error),

    #[error("unsupported zabbix api")]
    UnsupportedApiError(#[from] serde_json::Error),

    #[error("zabbix api bad request error")]
    BadRequestError,

    #[error("zabbix api error")]
    Error
}

#[derive(Deserialize)]
pub struct ZabbixError {
    pub code: i32,
    pub message: String,
    pub data: String
}