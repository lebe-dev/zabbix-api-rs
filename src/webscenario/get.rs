use serde::Serialize;

use crate::ZABBIX_EXTEND_PROPERTY_VALUE;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/get
#[derive(Serialize, Debug)]
pub struct GetWebScenarioByIdRequest {
    pub output: String,

    #[serde(rename = "selectSteps")]
    pub select_steps: String,

    #[serde(rename = "httptestids")]
    pub httptest_ids: String,
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/get
#[derive(Serialize, Debug)]
pub struct GetWebScenarioByNameRequest {
    pub output: String,

    #[serde(rename = "selectSteps")]
    pub select_steps: String,

    pub search: WebScenarioNameFilter,
}

impl GetWebScenarioByNameRequest {
    pub fn new(name: &str) -> GetWebScenarioByNameRequest {
        GetWebScenarioByNameRequest {
            output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            select_steps: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            search: WebScenarioNameFilter {
                name: name.to_string(),
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct WebScenarioNameFilter {
    pub name: String,
}
