use serde::{Deserialize, Serialize};

pub mod create;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object
#[derive(Deserialize,Debug)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String
}

// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object#host-tag
#[derive(Serialize,Deserialize,Debug)]
pub struct ZabbixHostTag {
    pub tag: String,
    pub value: String
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostinterface/object
#[derive(Serialize,Deserialize,Debug)]
pub struct ZabbixHostInterface {
    pub r#type: u8,

    pub main: u8,

    pub ip: String,

    pub dns: String,

    #[serde(rename = "useip")]
    pub use_ip: u8
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostgroup/object
#[derive(Serialize, Deserialize,Debug)]
pub struct ZabbixHostGroup {
    #[serde(rename = "groupid")]
    pub group_id: String,
}