use serde::{Deserialize, Serialize};

use crate::trigger::ZabbixTriggerTag;

#[derive(Serialize,Debug,Clone)]
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

#[derive(Serialize,Debug,Clone)]
pub struct ZabbixTriggerDependency {
    #[serde(alias = "triggerid")]
    pub trigger_id: String,
}

#[derive(Deserialize,Debug,Clone)]
pub struct CreateTriggerResponse {
    #[serde(rename = "triggerids")]
    pub trigger_ids: Vec<String>
}