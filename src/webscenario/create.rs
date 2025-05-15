use serde::{Deserialize, Serialize};

use super::model::ZabbixWebScenarioStep;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/create
#[derive(Serialize, Debug)]
pub struct CreateWebScenarioRequest {
    pub name: String,
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub steps: Vec<ZabbixWebScenarioStep>,
}

#[derive(Deserialize)]
pub struct CreateWebScenarioResponse {
    #[serde(rename = "httptestids")]
    pub http_test_ids: Vec<String>,
}
