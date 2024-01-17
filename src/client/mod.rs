use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::client::jsonrpc::ZabbixApiResponse;
use crate::error::ZabbixApiError;

pub mod jsonrpc;
mod v6;
mod post;

pub trait ZabbixApiClient {
    fn get_auth_session(&self, login: &str, token: &str) -> Result<String, ZabbixApiError>;
    fn raw_api_call<P: Serialize, R: DeserializeOwned>(&self, session: &str, method: &str, params: &P) -> Result<ZabbixApiResponse<R>, ZabbixApiError>;
}