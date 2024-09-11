use serde::{Deserialize, Serialize};

pub mod create;
pub mod get;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object
#[derive(Deserialize,PartialEq,Debug)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String
}

// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/object#host-tag
#[derive(Serialize,Deserialize,PartialEq,Debug)]
pub struct ZabbixHostTag {
    pub tag: String,
    pub value: String
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostinterface/object
#[derive(Serialize,Deserialize,PartialEq,Debug)]
pub struct ZabbixHostInterface {
    pub r#type: u8,

    pub main: u8,

    pub ip: String,

    pub dns: String,

    #[serde(rename = "useip")]
    pub use_ip: u8
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/hostgroup/object
#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct ZabbixHostGroup {
    pub name: String,
    #[serde(rename = "groupid")]
    pub group_id: String,
}