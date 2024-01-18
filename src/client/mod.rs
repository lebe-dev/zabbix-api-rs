use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::jsonrpc::ZabbixApiResponse;
use crate::error::ZabbixApiError;
use crate::host::{ZabbixHost, ZabbixHostGroup};
use crate::host::create::{CreateHostGroupRequest, CreateHostRequest};
use crate::item::create::CreateItemRequest;
use crate::item::ZabbixItem;
use crate::trigger::create::CreateTriggerRequest;
use crate::webscenario::create::CreateWebScenarioRequest;

pub mod jsonrpc;
pub mod v6;
pub mod post;

pub trait ZabbixApiClient {
    /// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/apiinfo/version
    fn get_api_info(&self) -> Result<String, ZabbixApiError>;

    fn get_auth_session(&self, login: &str, token: &str) -> Result<String, ZabbixApiError>;

    fn raw_api_call<P: Serialize, R: DeserializeOwned>(&self, session: &str, method: &str, params: &P) -> Result<ZabbixApiResponse<R>, ZabbixApiError>;

    fn get_host_groups<P: Serialize>(&self, session: &str, params: &P) -> Result<Vec<ZabbixHostGroup>, ZabbixApiError>;

    fn get_hosts<P: Serialize>(&self, session: &str, params: &P) -> Result<Vec<ZabbixHost>, ZabbixApiError>;

    fn get_items<P: Serialize>(&self, session: &str, params: &P) -> Result<Vec<ZabbixItem>, ZabbixApiError>;

    fn create_host_group(&self, session: &str, request: &CreateHostGroupRequest) -> Result<u32, ZabbixApiError>;

    fn create_host(&self, session: &str, request: &CreateHostRequest) -> Result<u32, ZabbixApiError>;

    fn create_item(&self, session: &str, request: &CreateItemRequest) -> Result<u32, ZabbixApiError>;

    fn create_trigger(&self, session: &str, request: &CreateTriggerRequest) -> Result<u32, ZabbixApiError>;

    fn create_webscenario(&self, session: &str, request: &CreateWebScenarioRequest) -> Result<u32, ZabbixApiError>;
}