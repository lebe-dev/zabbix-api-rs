use std::collections::HashMap;

use log::debug;
use log::error;
use log::info;
use reqwest::blocking::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::client::request::get_api_request;
use crate::error::ZabbixApiError;
use crate::host::create::CreateHostRequest;
use crate::host::create::CreateHostResponse;
use crate::host::model::ZabbixHost;
use crate::host::update::UpdateHostRequest;
use crate::host::update::UpdateHostResponse;
use crate::hostgroup::create::CreateHostGroupRequest;
use crate::hostgroup::model::ZabbixHostGroup;
use crate::item::create::CreateItemRequest;
use crate::item::create::CreateItemResponse;
use crate::item::model::ZabbixItem;
use crate::trigger::create::CreateTriggerRequest;
use crate::trigger::create::CreateTriggerResponse;
use crate::trigger::model::ZabbixTrigger;
#[cfg(feature = "user")]
use crate::user::create::{CreateUserRequest, CreateUserResponse};
#[cfg(feature = "user")]
use crate::user::model::ZabbixUser;
#[cfg(feature = "user")]
use crate::usergroup::model::{CreateUserGroupRequest, CreateUserGroupResponse, ZabbixUserGroup};
use crate::webscenario::create::CreateWebScenarioRequest;
use crate::webscenario::create::CreateWebScenarioResponse;
use crate::webscenario::model::ZabbixWebScenario;

use super::post::send_post_request;
use super::response::ZabbixApiResponse;

pub trait ZabbixApiClient {
    /// # get_api_info
    ///
    /// Retrieves the version of the Zabbix API.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/apiinfo/version
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    ///
    /// match client.get_api_info() {
    ///     Ok(version) => println!("Zabbix API Version: {}", version),
    ///     Err(e) => eprintln!("Error fetching API info: {:?}", e),
    /// }
    /// ```
    fn get_api_info(&self) -> Result<String, ZabbixApiError>;

    /// # get_auth_session
    ///
    /// Authenticates a user and returns a session token.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/user/login
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    ///
    /// match client.get_auth_session(&user, &password) {
    ///     Ok(session_token) => println!("Successfully authenticated. Session token: {}", session_token),
    ///     Err(e) => eprintln!("Authentication failed: {:?}", e),
    /// }
    /// ```
    fn get_auth_session(&self, login: &str, token: &str) -> Result<String, ZabbixApiError>;

    /// # raw_api_call
    ///
    /// Performs a generic Zabbix API call. This method is useful for calling API methods
    /// that are not yet explicitly supported by this client.
    ///
    /// - `session`: The authentication token.
    /// - `method`: The Zabbix API method to call (e.g., "host.get", "item.create").
    /// - `params`: The parameters for the API method, serializable to JSON.
    ///
    /// Returns a `ZabbixApiResponse` which includes the result or an error.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::client::response::ZabbixApiResponse;
    /// use serde::{Serialize, Deserialize}; // Combined and Deserialize is required for DeserializeOwned
    /// use std::collections::HashMap; // For a simple params map
    ///
    /// #[derive(Serialize)]
    /// struct HostGetParams {
    ///     output: String,
    ///     filter: HostFilter,
    /// }
    ///
    /// #[derive(Serialize)]
    /// struct HostFilter {
    ///     host: Vec<String>,
    /// }
    ///
    /// #[derive(Deserialize, Debug)] // Add Deserialize and Debug
    /// struct ZabbixHost {
    ///    hostid: String,
    ///    host: String,
    /// }
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let params = HostGetParams {
    ///     output: "extend".to_string(),
    ///     filter: HostFilter {
    ///         host: vec!["Zabbix server".to_string()],
    ///     },
    /// };
    ///
    /// match client.raw_api_call::<HostGetParams, Vec<ZabbixHost>>(&session, "host.get", &params) {
    ///     Ok(response) => {
    ///         if let Some(hosts) = response.result {
    ///             println!("Found hosts: {:?}", hosts);
    ///         } else if let Some(error) = response.error {
    ///             eprintln!("API Error: {:?}", error);
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Request Error: {:?}", e),
    /// }
    /// ```
    fn raw_api_call<P: Serialize, R: DeserializeOwned>(
        &self,
        session: &str,
        method: &str,
        params: &P,
    ) -> Result<ZabbixApiResponse<R>, ZabbixApiError>;

    /// # get_host_groups
    ///
    /// Retrieves Zabbix host groups based on the provided parameters.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/hostgroup/get
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::hostgroup::get::GetHostGroupsRequest;
    /// use zabbix_api::hostgroup::model::ZabbixHostGroup; // For type of host_groups
    /// use serde::Serialize;
    /// use std::collections::HashMap; // For an empty filter
    ///
    /// #[derive(Serialize)]
    /// struct EmptyFilter {} // More explicit for an empty filter struct if needed
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// // Example: Get all host groups, extend output
    /// let params = GetHostGroupsRequest {
    ///     output: "extend".to_string(),
    ///     filter: HashMap::<String, String>::new(), // Using HashMap as the filter type R
    /// };
    ///
    /// match client.get_host_groups(&session, &params) {
    ///     Ok(host_groups) => println!("Found host groups: {:?}", host_groups),
    ///     Err(e) => eprintln!("Error getting host groups: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "host")]
    fn get_host_groups<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixHostGroup>, ZabbixApiError>;

    /// # get_hosts
    ///
    /// Find zabbix hosts.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/host/get
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::host::get::GetHostsRequest;
    /// use zabbix_api::host::model::ZabbixHost;
    /// use serde::Serialize;
    /// use zabbix_api::client::client::{ZabbixApiClientImpl, ZabbixApiClient};
    ///
    /// #[derive(Serialize)]
    /// struct HostFilterParams {
    ///   pub host: Vec<String>
    /// }
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let request_params = GetHostsRequest {
    ///     filter: HostFilterParams {
    ///         host: vec!["Zabbix server".to_string()],
    ///     },
    ///     // output: Some("extend".to_string()), // Add if GetHostsRequest supports it
    /// };
    ///
    /// match client.get_hosts(&session, &request_params) {
    ///     Ok(hosts) => println!("Found hosts: {:?}", hosts),
    ///     Err(e) => eprintln!("Error getting hosts: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "host")]
    fn get_hosts<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixHost>, ZabbixApiError>;

    /// # get_items
    ///
    /// Find zabbix items.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/item/get
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::item::model::ZabbixItem; // For the type of items
    /// use serde::Serialize;
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// // Replace with a real host ID
    /// let host_id_to_query = "10084";
    ///
    /// #[derive(Serialize)]
    /// struct ItemParams {
    ///     output: String,
    ///     hostids: String,
    /// }
    ///
    /// let params = ItemParams {
    ///     output: "extend".to_string(),
    ///     hostids: host_id_to_query.to_string(),
    /// };
    ///
    /// match client.get_items(&session, &params) {
    ///     Ok(items) => println!("Found items: {:?}", items),
    ///     Err(e) => eprintln!("Error getting items: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "item")]
    fn get_items<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixItem>, ZabbixApiError>;

    /// # get_triggers
    ///
    /// Retrieves Zabbix triggers based on the provided parameters.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/trigger/get
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::trigger::model::ZabbixTrigger; // For the type of triggers
    /// use serde::Serialize;
    /// use std::collections::HashMap;
    ///
    /// #[derive(Serialize)]
    /// struct GetTriggersParams {
    ///     output: String,
    ///     select_hosts: String, // Example: "extend"
    ///     // Add other filters as needed, e.g. hostids, groupids, min_severity
    ///     // filter: HashMap<String, serde_json::Value>,
    ///     limit: Option<u32>,
    /// }
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let params = GetTriggersParams {
    ///     output: "extend".to_string(),
    ///     select_hosts: "extend".to_string(),
    ///     limit: Some(10),
    /// };
    ///
    /// match client.get_triggers(&session, &params) {
    ///     Ok(triggers) => println!("Found triggers: {:?}", triggers),
    ///     Err(e) => eprintln!("Error getting triggers: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "trigger")]
    fn get_triggers<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixTrigger>, ZabbixApiError>;

    /// # get_webscenarios
    ///
    /// Retrieves Zabbix web scenarios (HTTP tests) based on the provided parameters.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/httptest/get
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::webscenario::model::ZabbixWebScenario; // For the type of webscenarios
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct GetWebScenariosParams {
    ///     output: String,
    ///     select_steps: String, // "extend" to get steps
    ///     // hostids: Option<Vec<String>>,
    ///     // limit: Option<u32>,
    /// }
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let params = GetWebScenariosParams {
    ///     output: "extend".to_string(),
    ///     select_steps: "extend".to_string(),
    /// };
    ///
    /// match client.get_webscenarios(&session, &params) {
    ///     Ok(webscenarios) => println!("Found web scenarios: {:?}", webscenarios),
    ///     Err(e) => eprintln!("Error getting web scenarios: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "webscenario")]
    fn get_webscenarios<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixWebScenario>, ZabbixApiError>;

    /// # get_users
    ///
    /// Retrieves Zabbix users based on the provided parameters.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/user/get
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::user::model::ZabbixUser; // For the type of users
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct GetUsersParams {
    ///     output: String,
    ///     // filter: Option<UserFilter>, // Define UserFilter as needed
    ///     // select_mediatypes: Option<String>,
    /// }
    ///
    /// // #[derive(Serialize)]
    /// // struct UserFilter {
    /// //    alias: Vec<String>,
    /// // }
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let params = GetUsersParams {
    ///     output: "extend".to_string(),
    ///     // filter: Some(UserFilter { alias: vec!["Admin".to_string()] }),
    /// };
    ///
    /// match client.get_users(&session, &params) {
    ///     Ok(users) => println!("Found users: {:?}", users),
    ///     Err(e) => eprintln!("Error getting users: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "user")]
    fn get_users<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixUser>, ZabbixApiError>;

    /// # create_host_group
    ///
    /// Creates a new Zabbix host group.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/hostgroup/create
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::hostgroup::create::CreateHostGroupRequest;
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let new_group_name = "My New Host Group";
    /// let request = CreateHostGroupRequest {
    ///     name: new_group_name.to_string(),
    /// };
    ///
    /// match client.create_host_group(&session, &request) {
    ///     Ok(group_id) => println!("Successfully created host group '{}' with ID: {}", new_group_name, group_id),
    ///     Err(e) => eprintln!("Error creating host group: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "host")]
    fn create_host_group(
        &self,
        session: &str,
        request: &CreateHostGroupRequest,
    ) -> Result<u32, ZabbixApiError>;

    /// # create_host
    ///
    /// Create zabbix host.
    ///
    /// API: https://www.zabbix.com/documentation/7.0/en/manual/api/reference/host/create
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClientImpl, ZabbixApiClient};
    /// use zabbix_api::host::create::CreateHostRequest;
    /// use zabbix_api::hostgroup::model::ZabbixHostGroupId; // For specifying group
    /// use zabbix_api::host::model::ZabbixHostInterface;
    /// // Other optional fields in CreateHostRequest might need these:
    /// // use zabbix_api::host::model::ZabbixHostTag;
    /// // use zabbix_api::template::model::ZabbixTemplate;
    /// // use zabbix_api::r#macro::model::ZabbixHostMacro;
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// // Assume you have a host_group_id, e.g., from a previous call or configuration
    /// let known_host_group_id = "2"; // Example host group ID
    ///
    /// let new_host_name = "my-new-example-host".to_string();
    ///
    /// let create_host_params = CreateHostRequest {
    ///     host: new_host_name.clone(),
    ///     groups: vec![ZabbixHostGroupId { group_id: known_host_group_id.to_string() }],
    ///     interfaces: vec![ZabbixHostInterface { r#type: 1, main: 1, use_ip: 1, ip: "127.0.0.1".to_string(), dns: "".to_string(), port: "10050".to_string() }],
    ///     tags: vec![],
    ///     templates: vec![],
    ///     macros: vec![],
    ///     inventory_mode: 0, // 0: disabled, 1: automatic, -1: manual
    ///     inventory: HashMap::new(),
    ///     ..Default::default()
    /// };
    ///
    /// match client.create_host(&session, &create_host_params) {
    ///     Ok(host_id) => println!("Successfully created host '{}' with ID: {}", new_host_name, host_id),
    ///     Err(e) => eprintln!("Error creating host: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "host")]
    fn create_host(
        &self,
        session: &str,
        request: &CreateHostRequest,
    ) -> Result<u32, ZabbixApiError>;

    /// # update_host
    ///
    /// Update zabbix host.
    ///
    /// API: https://www.zabbix.com/documentation/7.0/en/manual/api/reference/host/update
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClientImpl, ZabbixApiClient};
    /// use zabbix_api::host::update::UpdateHostRequest;
    /// use zabbix_api::host::model::HostStatus;
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let host_id = "12".to_string();
    ///
    /// let update_host_params = UpdateHostRequest {
    ///     hostid: host_id.clone(),
    ///     status: HostStatus::Disabled,
    /// };
    ///
    /// match client.update_host(&session, &update_host_params) {
    ///     Ok(id) => println!("Successfully disabled host with ID: {}", id),
    ///     Err(e) => eprintln!("Error disabling host: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "host")]
    fn update_host(
        &self,
        session: &str,
        request: &UpdateHostRequest,
    ) -> Result<u32, ZabbixApiError>;

    /// # delete_host
    ///
    /// Delete zabbix host.
    ///
    /// API: https://www.zabbix.com/documentation/7.0/en/manual/api/reference/host/delete
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClientImpl, ZabbixApiClient};
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let host_id = "12";
    ///
    /// let delete_host_params = vec![host_id.to_string()];
    ///
    /// match client.delete_hosts(&session, &delete_host_params) {
    ///     Ok(ids) => println!("Successfully deleted hosts with IDs: {:?}", ids),
    ///     Err(e) => eprintln!("Error deleting hosts: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "host")]
    fn delete_hosts(
        &self,
        session: &str,
        host_ids: &Vec<String>,
    ) -> Result<Vec<String>, ZabbixApiError>;

    /// # create_item
    ///
    /// Creates a new Zabbix item.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/item/create
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::item::create::CreateItemRequest;
    /// use zabbix_api::host::model::ZabbixHostTag; // For ZabbixItemTag if used
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// // Replace with a real host ID
    /// let host_id_for_item = "10084";
    ///
    /// let request = CreateItemRequest {
    ///     name: "My New Item".to_string(),
    ///     key_: "my.new.item.key".to_string(),
    ///     host_id: host_id_for_item.to_string(),
    ///     r#type: 0, // Zabbix agent
    ///     value_type: 3, // Numeric (unsigned)
    ///     interface_id: "0".to_string(), // Use "0" if not specific interface, or a real one
    ///     delay: "30s".to_string(),
    ///     tags: vec![], // Example: vec![ZabbixHostTag { tag: "env".to_string(), value: "prod".to_string() }]
    ///     // Add other fields as required by your item type
    /// };
    ///
    /// match client.create_item(&session, &request) {
    ///     Ok(item_id) => println!("Successfully created item '{}' with ID: {}", request.name, item_id),
    ///     Err(e) => eprintln!("Error creating item: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "item")]
    fn create_item(
        &self,
        session: &str,
        request: &CreateItemRequest,
    ) -> Result<u32, ZabbixApiError>;

    /// # create_trigger
    ///
    /// Creates a new Zabbix trigger.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/trigger/create
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::trigger::create::{CreateTriggerRequest, ZabbixTriggerDependency};
    /// use zabbix_api::trigger::model::ZabbixTriggerTag; // For example fields
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// // This expression assumes a host named 'MyHost' and an item with key 'my.item.key' exists.
    /// // Replace with a valid expression for your environment.
    /// let trigger_expression = "last(/MyHost/my.item.key)<10".to_string();
    ///
    /// let request = CreateTriggerRequest {
    ///     description: "My New Trigger".to_string(),
    ///     expression: trigger_expression,
    ///     priority: 4, // High priority
    ///     // Optional fields:
    ///     recovery_mode: Some(0), // Expression-based recovery
    ///     recovery_expression: Some("last(/MyHost/my.item.key)>=10".to_string()),
    ///     url: Some("http://example.com/docs/my-trigger".to_string()),
    ///     event_name: None,
    ///     dependencies: vec![], // Example: vec![ZabbixTriggerDependency { trigger_id: "12345".to_string() }]
    ///     tags: vec![], // Example: vec![ZabbixTriggerTag { tag: "service".to_string(), value: "backend".to_string() }]
    /// };
    ///
    /// match client.create_trigger(&session, &request) {
    ///     Ok(trigger_id) => println!("Successfully created trigger '{}' with ID: {}", request.description, trigger_id),
    ///     Err(e) => eprintln!("Error creating trigger: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "trigger")]
    fn create_trigger(
        &self,
        session: &str,
        request: &CreateTriggerRequest,
    ) -> Result<u32, ZabbixApiError>;

    /// # create_webscenario
    ///
    /// Creates a new Zabbix web scenario (HTTP test).
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/httptest/create
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::webscenario::create::CreateWebScenarioRequest;
    /// use zabbix_api::webscenario::model::ZabbixWebScenarioStep; // Assuming this is the step model
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// // Replace with a real host ID that will own this web scenario
    /// let host_id_for_webscenario = "10084";
    ///
    /// let step1 = ZabbixWebScenarioStep {
    ///     name: "Check Homepage".to_string(),
    ///     url: "http://example.com".to_string(),
    ///     status_codes: "200".to_string(),
    ///     no: "1".to_string(), // Step number
    ///     // Add other step fields like required string, timeout, etc.
    /// };
    ///
    /// let request = CreateWebScenarioRequest {
    ///     name: "My Example Web Scenario".to_string(),
    ///     host_id: host_id_for_webscenario.to_string(),
    ///     steps: vec![step1],
    ///     // Add other scenario fields like agent, delay, retries, etc.
    /// };
    ///
    /// match client.create_webscenario(&session, &request) {
    ///     Ok(webscenario_id) => println!("Successfully created web scenario '{}' with ID: {}", request.name, webscenario_id),
    ///     Err(e) => eprintln!("Error creating web scenario: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "webscenario")]
    fn create_webscenario(
        &self,
        session: &str,
        request: &CreateWebScenarioRequest,
    ) -> Result<u32, ZabbixApiError>;

    /// # create_user_group
    ///
    /// Creates a new Zabbix user group.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/usergroup/create
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::usergroup::model::{CreateUserGroupRequest, UserGroupPermission, UserGroupUser};
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// // Assume host_group_id "2" (Linux servers) and user_id "1" (Admin) exist.
    /// // Replace with actual IDs from your Zabbix setup.
    /// let request = CreateUserGroupRequest {
    ///     name: "My New User Group".to_string(),
    ///     gui_access: Some(0), // System default
    ///     users_status: Some(0), // Enabled
    ///     hostgroup_rights: Some(vec![UserGroupPermission {
    ///         id: "2".to_string(), // Host group ID to grant permission to
    ///         permission: 2, // Read-only access
    ///     }]),
    ///     users: Some(vec![UserGroupUser {
    ///         user_id: "1".to_string(), // User ID to add to the group
    ///     }]),
    ///     debug_mode: None,
    ///     templategroup_rights: None, // Added missing field
    ///     tag_filters: None,
    /// };
    ///
    /// match client.create_user_group(&session, &request) {
    ///     Ok(user_group_id) => println!("Successfully created user group '{}' with ID: {}", request.name, user_group_id),
    ///     Err(e) => eprintln!("Error creating user group: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "user")]
    fn create_user_group(
        &self,
        session: &str,
        request: &CreateUserGroupRequest,
    ) -> Result<u32, ZabbixApiError>;

    /// # get_user_groups
    ///
    /// Retrieves Zabbix user groups based on the provided parameters.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/usergroup/get
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::usergroup::get::{GetUserGroupsRequest, UserGroupFilter}; // Assuming these structs exist
    /// use serde::Serialize;
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&user, &password).unwrap();
    ///
    /// let params = GetUserGroupsRequest {
    ///     output: Some("extend".to_string()),
    ///     filter: Some(UserGroupFilter {
    ///         name: Some(vec!["Zabbix administrators".to_string()]), // Example filter
    ///     }),
    ///     // Add other fields like selectUsers, selectRights, etc.
    ///     usrgrpids: None,
    ///     userids: None,
    ///     status: None,
    ///     select_users: Some("extend".to_string()),
    ///     select_rights: Some("extend".to_string()),
    /// };
    ///
    /// match client.get_user_groups(&session, &params) {
    ///     Ok(user_groups) => println!("Found user groups: {:?}", user_groups),
    ///     Err(e) => eprintln!("Error getting user groups: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "user")]
    fn get_user_groups<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixUserGroup>, ZabbixApiError>;

    /// # create_user
    ///
    /// Creates a new Zabbix user.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/user/create
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use reqwest::blocking::Client;
    /// use zabbix_api::client::client::{ZabbixApiClient, ZabbixApiClientImpl};
    /// use zabbix_api::user::create::{CreateUserRequest, UserGroupId}; // Assuming these structs exist
    ///
    /// let http_client = Client::new();
    /// let url = std::env::var("ZABBIX_API_URL").expect("ZABBIX_API_URL not set");
    /// let api_user = std::env::var("ZABBIX_API_USER").expect("ZABBIX_API_USER not set");
    /// let api_password = std::env::var("ZABBIX_API_PASSWORD").expect("ZABBIX_API_PASSWORD not set");
    ///
    /// let client = ZabbixApiClientImpl::new(http_client, &url);
    /// let session = client.get_auth_session(&api_user, &api_password).unwrap();
    ///
    /// // Assume user_group_id "7" (e.g., "Guests" or some other group) and role_id "1" (e.g., "User role") exist.
    /// // Replace with actual IDs from your Zabbix setup.
    /// let request = CreateUserRequest {
    ///     username: "newapiuser".to_string(),
    ///     passwd: "SecurePassword123!".to_string(),
    ///     roleid: "1".to_string(), // Role ID
    ///     usrgrps: vec![UserGroupId { usrgrpid: "7".to_string() }], // User group(s)
    ///     name: Some("New".to_string()),
    ///     surname: Some("API User".to_string()),
    ///     autologin: Some(0), // 0 = disabled, 1 = enabled
    ///     autologout: Some("1h".to_string()), // e.g., 1 hour
    ///     lang: Some("en_US".to_string()),
    ///     refresh: Some("30s".to_string()),
    ///     theme: Some("default".to_string()),
    ///     url: None, // User's homepage URL
    ///     user_type: None, // Added missing field (e.g., Some(1) for Zabbix admin, Some(2) for Zabbix super admin)
    ///     user_medias: None, // Media for notifications
    /// };
    ///
    /// match client.create_user(&session, &request) {
    ///     Ok(user_id) => println!("Successfully created user '{}' with ID: {}", request.username, user_id),
    ///     Err(e) => eprintln!("Error creating user: {:?}", e),
    /// }
    /// ```
    #[cfg(feature = "user")]
    fn create_user(
        &self,
        session: &str,
        request: &CreateUserRequest,
    ) -> Result<u32, ZabbixApiError>;
}

#[derive(Debug, Clone)]
pub struct ZabbixApiClientImpl {
    client: Client,
    api_endpoint_url: String,
}

impl ZabbixApiClientImpl {
    pub fn new(client: Client, api_endpoint_url: &str) -> ZabbixApiClientImpl {
        ZabbixApiClientImpl {
            client,
            api_endpoint_url: api_endpoint_url.to_string(),
        }
    }
}

impl ZabbixApiClient for ZabbixApiClientImpl {
    fn get_api_info(&self) -> Result<String, ZabbixApiError> {
        let params = HashMap::<String, String>::new();

        let api_request = get_api_request("apiinfo.version", params, None);

        match send_post_request(&self.client, &self.api_endpoint_url, None, api_request) {
            Ok(response_body) => {
                let response = serde_json::from_str::<ZabbixApiResponse<String>>(&response_body)?;

                match response.result {
                    Some(api_version) => {
                        info!("zabbix api version: '{api_version}'");
                        Ok(api_version)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    fn get_auth_session(&self, login: &str, token: &str) -> Result<String, ZabbixApiError> {
        info!("getting auth session for user '{login}'..");

        let params = HashMap::from([
            ("username".to_string(), login.to_string()),
            ("password".to_string(), token.to_string()),
        ]);

        let api_request = get_api_request("user.login", params, None);

        match send_post_request(&self.client, &self.api_endpoint_url, None, api_request) {
            Ok(response_body) => {
                let response = serde_json::from_str::<ZabbixApiResponse<String>>(&response_body)?;

                match response.result {
                    Some(session) => {
                        info!("auth ok");
                        Ok(session)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    fn raw_api_call<P: Serialize, R: DeserializeOwned>(
        &self,
        session: &str,
        method: &str,
        params: &P,
    ) -> Result<ZabbixApiResponse<R>, ZabbixApiError> {
        info!("calling api method '{method}'..");

        let api_request = get_api_request(method, params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<R>>(&response_body)?;

                match response.result {
                    Some(_) => {
                        info!("api method '{method}' has been successfully called");
                        Ok(response)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # get_host_groups
    ///
    /// Implements `ZabbixApiClient::get_host_groups`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "host")]
    fn get_host_groups<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixHostGroup>, ZabbixApiError> {
        info!("getting host groups with params");

        let api_request = get_api_request("hostgroup.get", params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<Vec<ZabbixHostGroup>>>(
                    &response_body,
                )?;

                match response.result {
                    Some(results) => {
                        info!("host groups found: {:?}", results);
                        Ok(results)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # get_hosts
    ///
    /// Implements `ZabbixApiClient::get_hosts`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "host")]
    fn get_hosts<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixHost>, ZabbixApiError> {
        info!("getting hosts with params");

        let api_request = get_api_request("host.get", params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<Vec<ZabbixHost>>>(&response_body)?;

                match response.result {
                    Some(results) => {
                        info!("hosts found: {:?}", results);
                        Ok(results)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # get_items
    ///
    /// Implements `ZabbixApiClient::get_items`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "item")]
    fn get_items<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixItem>, ZabbixApiError> {
        info!("getting items with params");

        let api_request = get_api_request("item.get", params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<Vec<ZabbixItem>>>(&response_body)?;

                match response.result {
                    Some(results) => {
                        info!("hosts found: {:?}", results);
                        Ok(results)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # get_triggers
    ///
    /// Implements `ZabbixApiClient::get_triggers`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "trigger")]
    fn get_triggers<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixTrigger>, ZabbixApiError> {
        info!("getting triggers..");

        let api_request = get_api_request("trigger.get", params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<Vec<ZabbixTrigger>>>(&response_body)?;

                match response.result {
                    Some(results) => {
                        info!("hosts found: {:?}", results);
                        Ok(results)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # get_webscenarios
    ///
    /// Implements `ZabbixApiClient::get_webscenarios`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "webscenario")]
    fn get_webscenarios<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixWebScenario>, ZabbixApiError> {
        info!("getting web-scenarios..");

        let api_request = get_api_request("httptest.get", params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<Vec<ZabbixWebScenario>>>(
                    &response_body,
                )?;

                match response.result {
                    Some(results) => {
                        info!("hosts found: {:?}", results);
                        Ok(results)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # create_host_group
    ///
    /// Implements `ZabbixApiClient::create_host_group`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "host")]
    fn create_host_group(
        &self,
        session: &str,
        request: &CreateHostGroupRequest,
    ) -> Result<u32, ZabbixApiError> {
        use crate::hostgroup::create::CreateHostGroupResponse;

        info!("creating host group '{}'..", request.name);

        let api_request = get_api_request("hostgroup.create", request, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<CreateHostGroupResponse>>(
                    &response_body,
                )?;

                match response.result {
                    Some(result) => {
                        info!("host group '{}' has been created", request.name);

                        match result.group_ids.first() {
                            Some(id) => id.parse::<u32>().map_err(|_| ZabbixApiError::Error),
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # create_host
    ///
    /// Implements `ZabbixApiClient::create_host`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "host")]
    fn create_host(
        &self,
        session: &str,
        request: &CreateHostRequest,
    ) -> Result<u32, ZabbixApiError> {
        info!("creating host '{}'..", request.host);

        let api_request = get_api_request("host.create", request, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<CreateHostResponse>>(&response_body)?;

                match response.result {
                    Some(result) => {
                        info!("host '{}' has been created", request.host);

                        match result.host_ids.first() {
                            Some(host_id) => {
                                host_id.parse::<u32>().map_err(|_| ZabbixApiError::Error)
                            }
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # update_host
    ///
    /// Implements `ZabbixApiClient::update_host`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "host")]
    fn update_host(
        &self,
        session: &str,
        request: &UpdateHostRequest,
    ) -> Result<u32, ZabbixApiError> {
        info!("updating host '{:?}'..", &serde_json::to_string(request));

        let api_request = get_api_request("host.update", request, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<UpdateHostResponse>>(&response_body)?;

                match response.result {
                    Some(result) => {
                        info!("host '{}' has been updated", request.hostid);

                        match result.host_ids.first() {
                            Some(host_id) => {
                                host_id.parse::<u32>().map_err(|_| ZabbixApiError::Error)
                            }
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # delete_host
    ///
    /// Implements `ZabbixApiClient::delete_host`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "host")]
    fn delete_hosts(
        &self,
        session: &str,
        host_ids: &Vec<String>,
    ) -> Result<Vec<String>, ZabbixApiError> {
        info!("deleting hosts '{:?}'..", &serde_json::to_string(host_ids));

        let api_request = get_api_request("host.delete", host_ids, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<UpdateHostResponse>>(&response_body)?;

                match response.result {
                    Some(result) => {
                        debug!("hosts '{:?}' have been deleted", result.host_ids);

                        Ok(result.host_ids)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # create_item
    ///
    /// Implements `ZabbixApiClient::create_item`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "item")]
    fn create_item(
        &self,
        session: &str,
        request: &CreateItemRequest,
    ) -> Result<u32, ZabbixApiError> {
        info!(
            "creating item with key '{}' for host id {}..",
            request.key_, request.host_id
        );

        let api_request = get_api_request("item.create", request, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<CreateItemResponse>>(&response_body)?;

                match response.result {
                    Some(result) => {
                        info!("item '{}' has been created", request.key_);

                        match result.item_ids.first() {
                            Some(host_id) => {
                                host_id.parse::<u32>().map_err(|_| ZabbixApiError::Error)
                            }
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # create_trigger
    ///
    /// Implements `ZabbixApiClient::create_trigger`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "trigger")]
    fn create_trigger(
        &self,
        session: &str,
        request: &CreateTriggerRequest,
    ) -> Result<u32, ZabbixApiError> {
        info!(
            "creating trigger '{}' with expression '{}'..",
            request.description, request.expression
        );

        let api_request = get_api_request("trigger.create", request, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<CreateTriggerResponse>>(
                    &response_body,
                )?;

                match response.result {
                    Some(result) => {
                        info!("trigger '{}' has been created", request.description);

                        match result.trigger_ids.first() {
                            Some(host_id) => {
                                host_id.parse::<u32>().map_err(|_| ZabbixApiError::Error)
                            }
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # create_webscenario
    ///
    /// Implements `ZabbixApiClient::create_webscenario`.
    ///
    /// See the trait documentation for more details.
    #[cfg(feature = "webscenario")]
    fn create_webscenario(
        &self,
        session: &str,
        request: &CreateWebScenarioRequest,
    ) -> Result<u32, ZabbixApiError> {
        info!(
            "creating web-scenario '{}' for host id '{}'..",
            request.name, request.host_id
        );

        let api_request = get_api_request("httptest.create", request, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<CreateWebScenarioResponse>>(
                    &response_body,
                )?;

                match response.result {
                    Some(result) => {
                        info!("web-scenario '{}' has been created", request.name);

                        match result.http_test_ids.first() {
                            Some(host_id) => {
                                host_id.parse::<u32>().map_err(|_| ZabbixApiError::Error)
                            }
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);

                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    #[cfg(feature = "user")]
    fn get_users<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixUser>, ZabbixApiError> {
        info!("getting users..");

        let api_request = get_api_request("user.get", params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(&session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<Vec<ZabbixUser>>>(&response_body)?;

                match response.result {
                    Some(results) => {
                        info!("users found: {:?}", results.len());
                        Ok(results)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);
                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    /// # create_user_group
    ///
    /// Implements `ZabbixApiClient::create_user_group`.
    ///
    /// API: https://www.zabbix.com/documentation/current/en/manual/api/reference/usergroup/create
    #[cfg(feature = "user")]
    fn create_user_group(
        &self,
        session: &str,
        request: &CreateUserGroupRequest,
    ) -> Result<u32, ZabbixApiError> {
        info!("creating user group '{}'..", request.name);

        let api_request = get_api_request("usergroup.create", request, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<CreateUserGroupResponse>>(
                    &response_body,
                )?;

                match response.result {
                    Some(result) => {
                        info!("user group '{}' has been created", request.name);

                        match result.user_group_ids.first() {
                            Some(id) => id.parse::<u32>().map_err(|_| ZabbixApiError::Error),
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);
                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    #[cfg(feature = "user")]
    fn get_user_groups<P: Serialize>(
        &self,
        session: &str,
        params: &P,
    ) -> Result<Vec<ZabbixUserGroup>, ZabbixApiError> {
        info!("getting user groups..");

        let api_request = get_api_request("usergroup.get", params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response = serde_json::from_str::<ZabbixApiResponse<Vec<ZabbixUserGroup>>>(
                    &response_body,
                )?;

                match response.result {
                    Some(results) => {
                        info!("user groups found: {:?}", results.len());
                        Ok(results)
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);
                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }

    #[cfg(feature = "user")]
    fn create_user(
        &self,
        session: &str,
        request: &CreateUserRequest,
    ) -> Result<u32, ZabbixApiError> {
        info!("creating user '{}'..", request.username);

        let params = [request];
        let api_request = get_api_request("user.create", &params, Some(session.to_string()));

        match send_post_request(
            &self.client,
            &self.api_endpoint_url,
            Some(session),
            api_request,
        ) {
            Ok(response_body) => {
                debug!("[response body]");
                debug!("{response_body}");
                debug!("[/response body]");

                let response =
                    serde_json::from_str::<ZabbixApiResponse<CreateUserResponse>>(&response_body)?;

                match response.result {
                    Some(result) => {
                        info!("user '{}' has been created", request.username);

                        match result.user_ids.first() {
                            Some(id) => id.parse::<u32>().map_err(|_| ZabbixApiError::Error),
                            None => {
                                error!("unexpected error, server returned empty id list");
                                Err(ZabbixApiError::Error)
                            }
                        }
                    }
                    None => match response.error {
                        Some(error) => {
                            error!("{:?}", error);
                            Err(ZabbixApiError::ApiCallError { zabbix: error })
                        }
                        None => Err(ZabbixApiError::BadRequestError),
                    },
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }
}

#[cfg(all(test, feature = "user"))]
mod user_tests {
    use log::{error, info};
    use serde::Serialize;
    use std::error::Error as StdError;

    use super::ZabbixApiClient;
    use crate::tests::builder::TestEnvBuilder;
    use crate::tests::integration::are_integration_tests_enabled;
    use crate::tests::logging::init_logging;
    use crate::tests::strings::get_random_string;
    use crate::user::create::{CreateUserRequest, UserGroupId};
    use crate::usergroup::get::{GetUserGroupsRequest, UserGroupFilter};
    use crate::usergroup::model::CreateUserGroupRequest;
    use crate::ZABBIX_EXTEND_PROPERTY_VALUE;

    #[test]
    fn get_users_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();
            test_env.get_session();

            #[derive(Serialize)]
            struct UserFilterParams {
                output: String,
                filter: UserFilter,
            }

            #[derive(Serialize)]
            struct UserFilter {
                alias: Vec<String>,
            }

            let api_user_alias = test_env.integration_tests_config.zabbix_api_user.clone();

            let params = UserFilterParams {
                output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
                filter: UserFilter {
                    alias: vec![api_user_alias.clone()],
                },
            };

            match test_env.client.get_users(&test_env.session, &params) {
                Ok(users) => {
                    assert!(!users.is_empty(), "Expected to find at least one user");
                    let found_user = users.iter().find(|u| u.alias == api_user_alias);
                    assert!(
                        found_user.is_some(),
                        "Expected to find user with alias '{}'",
                        api_user_alias
                    );
                    if let Some(user) = found_user {
                        info!("Successfully fetched user: {:?}", user);
                    }
                }
                Err(e) => {
                    error!("get_users test failed: {}", e);
                    if let Some(source) = e.source() {
                        error!("Caused by: {}", source);
                    }
                    panic!("get_users test failed");
                }
            }
        }
    }

    #[test]
    fn get_user_groups_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();
            test_env.get_session();

            let user_group_name = format!("test_get_ug_{}", get_random_string());
            let create_request = CreateUserGroupRequest {
                name: user_group_name.clone(),
                ..Default::default()
            };

            let user_group_id = test_env
                .client
                .create_user_group(&test_env.session, &create_request)
                .expect("Failed to create user group for get_user_groups_test");
            info!(
                "Created user group '{}' with ID '{}'",
                user_group_name, user_group_id
            );

            let get_request = GetUserGroupsRequest {
                output: Some(ZABBIX_EXTEND_PROPERTY_VALUE.to_string()),
                filter: Some(UserGroupFilter {
                    name: Some(vec![user_group_name.clone()]),
                }),
                select_users: Some("extend".to_string()),
                select_rights: Some("extend".to_string()),
                ..Default::default()
            };

            match test_env
                .client
                .get_user_groups(&test_env.session, &get_request)
            {
                Ok(user_groups) => {
                    assert!(
                        !user_groups.is_empty(),
                        "Expected to find at least one user group"
                    );
                    let found_group = user_groups.iter().find(|ug| ug.name == user_group_name);
                    assert!(
                        found_group.is_some(),
                        "Expected to find user group with name '{}'",
                        user_group_name
                    );
                    if let Some(group) = found_group {
                        info!("Successfully fetched user group: {:?}", group);
                        assert_eq!(group.usrgrp_id, user_group_id.to_string());
                    }
                }
                Err(e) => {
                    error!("get_user_groups test failed: {}", e);
                    if let Some(source) = e.source() {
                        error!("Caused by: {}", source);
                    }
                    panic!("get_user_groups test failed");
                }
            }
        }
    }

    #[test]
    fn create_user_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();
            test_env.get_session();

            let user_group_name = format!("test_ug_for_user_create_{}", get_random_string());
            let create_ug_request = CreateUserGroupRequest {
                name: user_group_name.clone(),
                ..Default::default()
            };

            let user_group_id = test_env
                .client
                .create_user_group(&test_env.session, &create_ug_request)
                .expect("Failed to create user group for create_user_test");
            info!(
                "Created user group '{}' with ID '{}' for create_user_test",
                user_group_name, user_group_id
            );

            let user_alias = format!("test_user_{}", get_random_string());
            let user_passwd = get_random_string();
            let role_id = "3"; // Default "User role" ID

            let create_user_req = CreateUserRequest {
                username: user_alias.clone(),
                passwd: user_passwd,
                roleid: role_id.to_string(),
                usrgrps: vec![UserGroupId {
                    usrgrpid: user_group_id.to_string(),
                }],
                name: Some("Test".to_string()),
                surname: Some("User".to_string()),
                ..Default::default()
            };

            match test_env
                .client
                .create_user(&test_env.session, &create_user_req)
            {
                Ok(user_id) => {
                    assert!(user_id > 0, "Expected a valid user ID to be returned");
                    info!(
                        "Successfully created user '{}' with ID '{}'",
                        user_alias, user_id
                    );
                }
                Err(e) => {
                    error!("create_user test failed: {}", e);
                    if let Some(source) = e.source() {
                        error!("Caused by: {}", source);
                    }
                    panic!("create_user test failed");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use log::{error, info};
    use reqwest::blocking::Client;
    use serde::Serialize;

    use crate::client::client::ZabbixApiClient;
    use crate::host::create::TlsConfig;
    use crate::host::get::GetHostsRequest;
    use crate::host::model::ZabbixHost;
    use crate::host::update::UpdateHostRequest;
    use crate::hostgroup::get::GetHostGroupsRequest;
    use crate::item::create::CreateItemRequest;
    use crate::item::get::GetItemsRequestById;
    use crate::tests::builder::TestEnvBuilder;
    use crate::tests::integration::{are_integration_tests_enabled, get_integration_tests_config};
    use crate::tests::logging::init_logging;
    use crate::tests::strings::get_random_hex_string;
    use crate::tests::strings::get_random_string;
    use crate::trigger::create::CreateTriggerRequest;
    use crate::trigger::get::GetTriggerByIdRequest;
    use crate::usergroup::model::{CreateUserGroupRequest, UserGroupPermission, UserGroupUser};
    use crate::webscenario::create::CreateWebScenarioRequest;
    use crate::webscenario::get::GetWebScenarioByIdRequest;
    use crate::webscenario::model::ZabbixWebScenarioStep;
    use crate::ZABBIX_EXTEND_PROPERTY_VALUE;

    use super::ZabbixApiClientImpl;

    #[test]
    fn get_api_info() {
        if are_integration_tests_enabled() {
            let test_env = TestEnvBuilder::build();

            match test_env.client.get_api_info() {
                Ok(result) => {
                    assert!(!result.is_empty())
                }
                Err(e) => {
                    error!("error: {}", e);
                    panic!("unexpected error")
                }
            }
        }
    }

    #[test]
    fn session_should_be_returned() {
        init_logging();

        if are_integration_tests_enabled() {
            let http_client = Client::new();

            let tests_config = get_integration_tests_config();

            let client = ZabbixApiClientImpl::new(http_client, &tests_config.zabbix_api_url);

            match client.get_auth_session(
                &tests_config.zabbix_api_user,
                &tests_config.zabbix_api_password,
            ) {
                Ok(session) => assert!(session.len() > 0),
                Err(e) => {
                    error!("error: {}", e);
                    panic!("unexpected error")
                }
            }
        }
    }

    #[test]
    fn raw_api_call_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();
            test_env.get_session();

            #[derive(Serialize)]
            struct Params {
                pub filter: Filter,
            }

            #[derive(Serialize)]
            struct Filter {
                pub host: Vec<String>,
            }

            let params = Params {
                filter: Filter {
                    host: vec!["Zabbix server".to_string()],
                },
            };

            match test_env.client.raw_api_call::<Params, Vec<ZabbixHost>>(
                &test_env.session,
                "host.get",
                &params,
            ) {
                Ok(response) => {
                    let results = response.result.unwrap();
                    info!("{:?}", results.first().unwrap());
                    assert_eq!(1, results.len())
                }
                Err(e) => {
                    error!("api call error: {}", e);
                    panic!("unexpected api call error")
                }
            }
        }
    }

    #[test]
    fn get_host_groups_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let group_name2 = get_random_string();
            let group_name3 = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host_group(&group_name2)
                .create_host_group(&group_name3);

            #[derive(Serialize)]
            struct Filter {
                pub name: Vec<String>,
            }

            let request = GetHostGroupsRequest {
                output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
                filter: Filter {
                    name: vec![group_name2.to_string()],
                },
            };

            match test_env.client.get_host_groups(&test_env.session, &request) {
                Ok(host_groups) => {
                    assert_eq!(host_groups.len(), 1);

                    let host_group = host_groups.first().unwrap();

                    assert_eq!(&host_group.name, &group_name2)
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("host group get error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn get_hosts_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name1 = get_random_string();
            let host_name2 = get_random_string();
            let host_name3 = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name1, None)
                .create_host(&host_name2, None)
                .create_host(&host_name3, None);

            #[derive(Serialize)]
            struct Filter {
                pub host: Vec<String>,
            }

            let request = GetHostsRequest {
                filter: Filter {
                    host: vec![host_name2.to_string()],
                },
            };

            match test_env.client.get_hosts(&test_env.session, &request) {
                Ok(hosts) => {
                    assert_eq!(hosts.len(), 1);

                    let host = hosts.first().unwrap();

                    assert_eq!(&host.host, &host_name2)
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("host get error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn delete_hosts_test() {
        init_logging();

        #[derive(Serialize)]
        struct HostFilter {
            pub host: Vec<String>,
        }

        if are_integration_tests_enabled() {
            use crate::host::get::GetHostsByIdsRequest;
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name1 = get_random_string();
            let host_name2 = get_random_string();
            let host_name3 = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name1, None)
                .create_host(&host_name2, None)
                .create_host(&host_name3, None);

            match test_env.client.get_hosts(
                &test_env.session,
                &GetHostsRequest {
                    filter: HostFilter {
                        host: vec![host_name1, host_name2, host_name3],
                    },
                },
            ) {
                Ok(hosts) => {
                    let host_ids = hosts
                        .iter()
                        .map(|host| host.host_id.clone())
                        .collect::<Vec<String>>();

                    test_env.get_session().delete_hosts(&host_ids);

                    match test_env.client.get_hosts(
                        &test_env.session,
                        &GetHostsByIdsRequest { hostids: host_ids },
                    ) {
                        Ok(hosts) => {
                            assert!(hosts.is_empty());
                        }
                        Err(e) => {
                            if let Some(inner_source) = e.source() {
                                println!("Caused by: {}", inner_source);
                            }

                            error!("host get error: {}", e);
                            panic!("{}", e)
                        }
                    }
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("host get error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn get_items_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name1 = get_random_string();
            let host_name2 = get_random_string();
            let host_name3 = get_random_string();
            let item_name = get_random_string();
            let item_key = format!("test{}", get_random_string());

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name1, None)
                .create_host(&host_name2, None)
                .create_host(&host_name3, None)
                .create_item(&item_name, &item_key);

            #[derive(Serialize)]
            struct Search {
                pub key_: String,
            }

            let request = GetItemsRequestById {
                output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
                with_triggers: false,
                host_ids: test_env.latest_host_id.to_string(),
                search: Search {
                    key_: item_key.to_string(),
                },
                sort_field: "name".to_string(),
            };

            match test_env.client.get_items(&test_env.session, &request) {
                Ok(items) => {
                    assert_eq!(items.len(), 1);

                    let item = items.first().unwrap();

                    assert_eq!(&item.key_, &item_key)
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("host get error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn get_triggers_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();
            let item_name = get_random_string();
            let item_key = get_random_string();
            let trigger_description = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name, None)
                .create_item(&item_name, &item_key)
                .create_trigger(
                    &trigger_description,
                    &format!("last(/{host_name}/{item_key})=0"),
                );

            let request = GetTriggerByIdRequest {
                trigger_ids: test_env.latest_trigger_id.to_string(),
                output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
                select_functions: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            };

            match test_env.client.get_triggers(&test_env.session, &request) {
                Ok(results) => {
                    assert_eq!(results.len(), 1);
                    let result = results.first().unwrap();

                    assert_eq!(&result.description, &trigger_description)
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("host get error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn get_webscenarios_test() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();
            let item_name = get_random_string();
            let item_key = get_random_string();
            let trigger_description = get_random_string();
            let webscenario_name = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name, None)
                .create_item(&item_name, &item_key)
                .create_trigger(
                    &trigger_description,
                    &format!("last(/{host_name}/{item_key})=0"),
                )
                .create_web_scenario(&webscenario_name);

            let request = GetWebScenarioByIdRequest {
                output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
                select_steps: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
                httptest_ids: test_env.latest_webscenario_id.to_string(),
            };

            match test_env
                .client
                .get_webscenarios(&test_env.session, &request)
            {
                Ok(results) => {
                    assert_eq!(results.len(), 1);
                    let result = results.first().unwrap();

                    assert_eq!(&result.name, &webscenario_name)
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("host get error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn create_host_group_and_host() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name, None);

            assert!(test_env.latest_host_group_id > 0);
            assert!(test_env.latest_host_id > 0);
        }
    }

    #[test]
    fn create_host_with_cert() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(
                    &host_name,
                    Some(TlsConfig::new_cert(
                        "CN=some issuer".to_string(),
                        "CN=some subject".to_string(),
                    )),
                );

            assert!(test_env.latest_host_group_id > 0);
            assert!(test_env.latest_host_id > 0);
        }
    }

    #[test]
    fn create_host_with_psk() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(
                    &host_name,
                    Some(TlsConfig::new_psk(
                        get_random_string(),
                        get_random_hex_string(),
                    )),
                );

            assert!(test_env.latest_host_group_id > 0);
            assert!(test_env.latest_host_id > 0);
        }
    }

    #[test]
    fn create_and_update_host() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name, None);

            assert!(test_env.latest_host_group_id > 0);
            assert!(test_env.latest_host_id > 0);

            let host_id = test_env.latest_host_id.to_string();
            test_env
                .get_session()
                .update_host(UpdateHostRequest::disable_host(host_id));
        }
    }

    #[test]
    fn create_item() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name, None);

            let item_key = get_random_string();
            let item_name = get_random_string();

            let request = CreateItemRequest {
                key_: item_key,
                name: item_name,
                host_id: test_env.latest_host_id.to_string(),
                r#type: 7,
                value_type: 4,
                interface_id: "0".to_string(),
                tags: vec![],
                delay: "30s".to_string(),
            };

            match test_env.client.create_item(&test_env.session, &request) {
                Ok(item_id) => {
                    assert!(item_id > 0);
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("item create error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn create_trigger() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();

            let item_name = get_random_string();
            let item_key = format!("key{}", get_random_string());

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name, None)
                .create_item(&item_name, &item_key);

            let trigger_description = get_random_string();

            let expression = format!("last(/{host_name}/{item_key})=0");

            let request = CreateTriggerRequest {
                description: trigger_description,
                expression: expression.to_string(),
                priority: 4,
                recovery_mode: Some(0),
                recovery_expression: None,
                url: None,
                event_name: None,
                dependencies: vec![],
                tags: vec![],
            };

            match test_env.client.create_trigger(&test_env.session, &request) {
                Ok(trigger_id) => assert!(trigger_id > 0),
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("trigger create error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn create_web_scenario() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();

            let group_name = get_random_string();
            let host_name = get_random_string();

            test_env
                .get_session()
                .create_host_group(&group_name)
                .create_host(&host_name, None);

            let web_scenario_name = get_random_string();

            let step = ZabbixWebScenarioStep {
                name: "Check github.com page".to_string(),
                url: "https://github.com".to_string(),
                status_codes: "200".to_string(),
                no: "0".to_string(),
            };

            let request = CreateWebScenarioRequest {
                name: web_scenario_name,
                host_id: test_env.latest_host_id.to_string(),
                steps: vec![step],
            };

            match test_env
                .client
                .create_webscenario(&test_env.session, &request)
            {
                Ok(web_scenario_id) => {
                    assert!(web_scenario_id > 0);
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }

                    error!("web-scenario create error: {}", e);
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    fn create_user_group() {
        init_logging();

        if are_integration_tests_enabled() {
            let mut test_env = TestEnvBuilder::build();
            test_env.get_session();

            let group_name = get_random_string();
            let user_group_name = format!("user_group_{}", get_random_string());

            // Create a host group to assign permissions to
            test_env.create_host_group(&group_name);
            let host_group_id = test_env.latest_host_group_id.to_string();

            // A dummy user ID (replace with a real one if needed for more thorough testing)
            // For this test, Zabbix might not validate the user ID existence strictly for group creation.
            let user_id = "1"; // Assuming user with ID '1' (Admin) exists or is not strictly checked

            let request = CreateUserGroupRequest {
                name: user_group_name.clone(),
                gui_access: Some(0),   // System default
                users_status: Some(0), // Enabled
                hostgroup_rights: Some(vec![UserGroupPermission {
                    id: host_group_id,
                    permission: 2, // Read-only
                }]),
                users: Some(vec![UserGroupUser {
                    user_id: user_id.to_string(),
                }]),
                ..Default::default()
            };

            match test_env
                .client
                .create_user_group(&test_env.session, &request)
            {
                Ok(user_group_id) => {
                    assert!(user_group_id > 0);
                    info!(
                        "Successfully created user group '{}' with ID '{}'",
                        user_group_name, user_group_id
                    );
                }
                Err(e) => {
                    if let Some(inner_source) = e.source() {
                        println!("Caused by: {}", inner_source);
                    }
                    error!("user group create error: {}", e);
                    panic!("{}", e);
                }
            }
        }
    }
}
