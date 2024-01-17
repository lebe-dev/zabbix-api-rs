use serde::{Deserialize, Serialize};

use crate::host::ZabbixHostTag;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/item/create
#[derive(Serialize,Debug)]
pub struct CreateItemRequest {
    pub name: String,
    pub key_: String,
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub r#type: u8,
    pub value_type: u8,
    #[serde(rename = "interfaceid")]
    pub interface_id: String,
    pub tags: Vec<ZabbixHostTag>,
    pub delay: String
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/item/create
#[derive(Deserialize,Debug)]
pub struct CreateItemResponse {
    #[serde(rename = "itemids")]
    pub item_ids: Vec<String>
}