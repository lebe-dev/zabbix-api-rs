use serde::Deserialize;
use crate::error::ZabbixApiError;

pub mod jsonrpc;
mod v6;
mod post;

pub trait ZabbixApiClient {
    fn get_auth_session(&self, login: &str, token: &str) -> Result<String, ZabbixApiError>;
}