use serde::{Deserialize, Serialize};

use super::model::ZabbixTriggerTag;

#[derive(Serialize, Debug, Clone)]
pub struct CreateTriggerRequest {
    pub description: String,
    pub expression: String,
    pub priority: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_mode: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_expression: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,

    pub dependencies: Vec<ZabbixTriggerDependency>,
    pub tags: Vec<ZabbixTriggerTag>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ZabbixTriggerDependency {
    #[serde(alias = "triggerid")]
    pub trigger_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTriggerResponse {
    #[serde(rename = "triggerids")]
    pub trigger_ids: Vec<String>,
}
