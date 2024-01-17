use serde::Deserialize;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/item/object
#[derive(Deserialize, Debug)]
pub struct ZabbixItem {
    pub name: String,

    pub key_: String,

    #[serde(rename = "hostid")]
    pub host_id: String
}