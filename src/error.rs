use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZabbixApiError {
    #[error("zabbix api error")]
    Error
}

#[derive(Deserialize)]
pub struct ZabbixError {
    pub code: i32,
    pub message: String,
    pub data: String
}