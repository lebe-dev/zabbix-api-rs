use serde::{Deserialize, Serialize};

use crate::trigger::ZabbixTriggerTag;

#[derive(Serialize)]
pub struct CreateTriggerRequest {
    pub description: String,
    pub expression: String,
    pub priority: u8,
    pub recovery_mode: u8,
    pub recovery_expression: String,
    pub url: String,
    pub event_name: String,
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