use serde::Deserialize;

pub mod create;
pub mod search;

#[derive(Deserialize, Clone, Debug)]
pub struct ZabbixTrigger {
    #[serde(alias = "triggerid")]
    pub trigger_id: String,
    #[serde(alias = "description")]
    pub name: String,
    pub expression: String
}