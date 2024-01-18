use serde::{Deserialize, Serialize};

pub mod create;
pub mod get;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/object
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct ZabbixWebScenario {
    pub name: String,
    #[serde(alias = "hostid")]
    pub host_id: String,
    pub steps: Vec<ZabbixWebScenarioStep>
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/object
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct ZabbixWebScenarioStep {
    pub name: String,
    pub url: String,
    pub status_codes: String,
    pub no: String,
}