use serde::{Deserialize, Serialize};

use crate::trigger::ZabbixTriggerTag;

#[derive(Serialize)]
pub struct CreateTriggerRequest {
    pub description: String,
    pub expression: String,
    pub priority: u8,
    pub recovery_mode: Option<u8>,
    pub recovery_expression: Option<String>,
    pub url: Option<String>,
    pub event_name: Option<String>,
    pub dependencies: Vec<ZabbixTriggerDependency>,
    pub tags: Vec<ZabbixTriggerTag>
}

#[derive(Serialize)]
pub struct ZabbixTriggerDependency {
    #[serde(alias = "triggerid")]
    pub trigger_id: String,
}

#[derive(Deserialize)]
pub struct CreateTriggerResponse {
    #[serde(rename = "triggerids")]
    pub trigger_ids: Vec<String>
}