use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::jsonrpc::ZabbixApiResponse;
use crate::error::ZabbixApiError;
use crate::host::create::{CreateHostGroupRequest, CreateHostRequest};
use crate::item::create::CreateItemRequest;

pub mod jsonrpc;
pub mod v6;
pub mod post;

pub trait ZabbixApiClient {
    fn get_auth_session(&self, login: &str, token: &str) -> Result<String, ZabbixApiError>;

    fn raw_api_call<P: Serialize, R: DeserializeOwned>(&self, session: &str, method: &str, params: &P) -> Result<ZabbixApiResponse<R>, ZabbixApiError>;

    fn create_host_group(&self, session: &str, request: &CreateHostGroupRequest) -> Result<u32, ZabbixApiError>;

    fn create_host(&self, session: &str, request: &CreateHostRequest) -> Result<u32, ZabbixApiError>;

    fn create_item(&self, session: &str, request: &CreateItemRequest) -> Result<u32, ZabbixApiError>;
}