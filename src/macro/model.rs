use serde::{Deserialize, Serialize};
use crate::r#macro::macrotype::MacroType;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/usermacro/object
#[derive(Deserialize, Debug)]
pub struct ZabbixGlobalMacro {
    #[serde(rename = "globalmacroid")]
    pub id: String,
    pub r#macro: String,
    pub value: String,
    pub r#type: u8,
    pub description: String,
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/usermacro/object
#[derive(Serialize, Deserialize, Debug)]
pub struct ZabbixHostMacro {
    #[serde(rename = "hostmacroid")]
    pub id: String,
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub r#macro: String,
    pub value: String,
    pub r#type: MacroType,
    pub description: String,
}
