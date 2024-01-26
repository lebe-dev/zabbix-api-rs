use serde::Serialize;

use crate::ZABBIX_EXTEND_PROPERTY_VALUE;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/trigger/get
#[derive(Serialize,Debug)]
pub struct GetTriggerByIdRequest {
    /// Trigger ID
    #[serde(rename = "triggerids")]
    pub trigger_ids: String,
    pub output: String,
    #[serde(rename = "selectFunctions")]
    pub select_functions: String
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/trigger/get
#[derive(Serialize,Debug)]
pub struct GetTriggerByDescriptionRequest {
    pub search: TriggerNameSearch,
    pub output: String,
    #[serde(rename = "selectFunctions")]
    pub select_functions: String
}

impl GetTriggerByDescriptionRequest {
    pub fn new(description: &str) -> GetTriggerByDescriptionRequest {
        GetTriggerByDescriptionRequest {
            search: TriggerNameSearch {
                description: description.to_string()
            },
            output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            select_functions: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
        }
    }
}

#[derive(Serialize,Debug)]
pub struct TriggerNameSearch {
    pub description: String,
}