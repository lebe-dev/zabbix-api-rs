use serde::Serialize;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/get
#[derive(Serialize,Debug)]
pub struct GetWebScenarioByIdRequest {
    pub output: String,

    #[serde(rename = "selectSteps")]
    pub select_steps: String,

    #[serde(rename = "httptestids")]
    pub httptest_ids: String,
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/get
#[derive(Serialize,Debug)]
pub struct GetWebScenarioByNameRequest {
    pub output: String,

    #[serde(rename = "selectSteps")]
    pub select_steps: String,

    pub search: WebScenarioNameFilter,
}

impl GetWebScenarioByNameRequest {
    pub fn new(name: &str) -> GetWebScenarioByNameRequest {
        GetWebScenarioByNameRequest {
            output: "extend".to_string(),
            select_steps: "extend".to_string(),
            search: WebScenarioNameFilter {
                name: name.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
struct WebScenarioNameFilter {
    pub name: String
}