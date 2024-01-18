use serde::Serialize;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/httptest/get
#[derive(Serialize,Debug)]
pub struct GetWebScenarioRequest {
    pub output: String,

    #[serde(rename = "selectSteps")]
    pub select_steps: String,

    #[serde(rename = "httptestids")]
    pub httptest_ids: String,
}