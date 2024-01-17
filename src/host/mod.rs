use serde::Deserialize;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object
#[derive(Deserialize,Debug)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String
}