use serde::{Deserialize, Serialize};

pub mod create;
pub mod search;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/trigger/object
#[derive(Deserialize, Clone, Debug)]
pub struct ZabbixTrigger {
    #[serde(alias = "triggerid")]
    pub trigger_id: String,
    #[serde(alias = "description")]
    pub name: String,
    pub expression: String
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/trigger/object#trigger-tag
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ZabbixTriggerTag {
    pub tag: String,
    pub value: String,
}